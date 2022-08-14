const std = @import("std");

const ParseError = error{DidNotFind};

const Definitions = struct {
    const Space = SingleVal('\x20');
    const Control = OneOf(&[_]type{
        SingleVal('\x00'), SingleVal('\x01'), SingleVal('\x02'), SingleVal('\x03'), SingleVal('\x04'), SingleVal('\x05'), SingleVal('\x06'), SingleVal('\x07'),
        SingleVal('\x08'), SingleVal('\x09'), SingleVal('\x0a'), SingleVal('\x0b'), SingleVal('\x0c'), SingleVal('\x0d'), SingleVal('\x0e'), SingleVal('\x0f'),
        SingleVal('\x10'), SingleVal('\x11'), SingleVal('\x12'), SingleVal('\x13'), SingleVal('\x14'), SingleVal('\x15'), SingleVal('\x16'), SingleVal('\x17'),
        SingleVal('\x18'), SingleVal('\x19'), SingleVal('\x1a'), SingleVal('\x1b'), SingleVal('\x1c'), SingleVal('\x1d'), SingleVal('\x1e'), SingleVal('\x1f'),
        SingleVal('\x7f'),
    });
    const Cr = SingleVal('\x0d');
    const Lf = SingleVal('\x0a');
    const CrLf = SingleVal(Cr ++ Lf);
    const Alpha = OneOf(&[_]type{
        SingleVal('\x41'), SingleVal('\x42'), SingleVal('\x43'), SingleVal('\x44'), SingleVal('\x45'), SingleVal('\x46'), SingleVal('\x47'), SingleVal('\x48'),
        SingleVal('\x49'), SingleVal('\x4a'), SingleVal('\x4b'), SingleVal('\x4c'), SingleVal('\x4d'), SingleVal('\x4e'), SingleVal('\x4f'), SingleVal('\x50'),
        SingleVal('\x51'), SingleVal('\x52'), SingleVal('\x53'), SingleVal('\x54'), SingleVal('\x55'), SingleVal('\x56'), SingleVal('\x57'), SingleVal('\x58'),
        SingleVal('\x59'), SingleVal('\x5a'), SingleVal('\x61'), SingleVal('\x62'), SingleVal('\x63'), SingleVal('\x64'), SingleVal('\x65'), SingleVal('\x66'),
        SingleVal('\x67'), SingleVal('\x68'), SingleVal('\x69'), SingleVal('\x6a'), SingleVal('\x6b'), SingleVal('\x6c'), SingleVal('\x6d'), SingleVal('\x6e'),
        SingleVal('\x6f'), SingleVal('\x70'), SingleVal('\x71'), SingleVal('\x72'), SingleVal('\x73'), SingleVal('\x74'), SingleVal('\x75'), SingleVal('\x76'),
        SingleVal('\x77'), SingleVal('\x78'), SingleVal('\x79'), SingleVal('\x7a'),
    });
    const Digit = OneOf(&[_]type{
        SingleVal('\x30'), SingleVal('\x31'), SingleVal('\x32'), SingleVal('\x33'), SingleVal('\x34'), SingleVal('\x35'), SingleVal('\x36'), SingleVal('\x37'), SingleVal('\x38'), SingleVal('\x39'),
    });
    const DQuote = SingleVal('\x22');
    const Octet = OneOf([_]u8{
        '\x00', '\x01', '\x02', '\x03', '\x04', '\x05', '\x06', '\x07', '\x08', '\x09', '\x0a', '\x0b', '\x0c', '\x0d', '\x0e', '\x0f',
        '\x10', '\x11', '\x12', '\x13', '\x14', '\x15', '\x16', '\x17', '\x18', '\x19', '\x1a', '\x1b', '\x1c', '\x1d', '\x1e', '\x1f',
        '\x20', '\x21', '\x22', '\x23', '\x24', '\x25', '\x26', '\x27', '\x28', '\x29', '\x2a', '\x2b', '\x2c', '\x2d', '\x2e', '\x2f',
        '\x30', '\x31', '\x32', '\x33', '\x34', '\x35', '\x36', '\x37', '\x38', '\x39', '\x3a', '\x3b', '\x3c', '\x3d', '\x3e', '\x3f',
        '\x40', '\x41', '\x42', '\x43', '\x44', '\x45', '\x46', '\x47', '\x48', '\x49', '\x4a', '\x4b', '\x4c', '\x4d', '\x4e', '\x4f',
        '\x50', '\x51', '\x52', '\x53', '\x54', '\x55', '\x56', '\x57', '\x58', '\x59', '\x5a', '\x5b', '\x5c', '\x5d', '\x5e', '\x5f',
        '\x60', '\x61', '\x62', '\x63', '\x64', '\x65', '\x66', '\x67', '\x68', '\x69', '\x6a', '\x6b', '\x6c', '\x6d', '\x6e', '\x6f',
        '\x70', '\x71', '\x72', '\x73', '\x74', '\x75', '\x76', '\x77', '\x78', '\x79', '\x7a', '\x7b', '\x7c', '\x7d', '\x7e', '\x7f',
        '\x80', '\x81', '\x82', '\x83', '\x84', '\x85', '\x86', '\x87', '\x88', '\x89', '\x8a', '\x8b', '\x8c', '\x8d', '\x8e', '\x8f',
        '\x90', '\x91', '\x92', '\x93', '\x94', '\x95', '\x96', '\x97', '\x98', '\x99', '\x9a', '\x9b', '\x9c', '\x9d', '\x9e', '\x9f',
        '\xa0', '\xa1', '\xa2', '\xa3', '\xa4', '\xa5', '\xa6', '\xa7', '\xa8', '\xa9', '\xaa', '\xab', '\xac', '\xad', '\xae', '\xaf',
        '\xb0', '\xb1', '\xb2', '\xb3', '\xb4', '\xb5', '\xb6', '\xb7', '\xb8', '\xb9', '\xba', '\xbb', '\xbc', '\xbd', '\xbe', '\xbf',
        '\xc0', '\xc1', '\xc2', '\xc3', '\xc4', '\xc5', '\xc6', '\xc7', '\xc8', '\xc9', '\xca', '\xcb', '\xcc', '\xcd', '\xce', '\xcf',
        '\xd0', '\xd1', '\xd2', '\xd3', '\xd4', '\xd5', '\xd6', '\xd7', '\xd8', '\xd9', '\xda', '\xdb', '\xdc', '\xdd', '\xde', '\xdf',
        '\xe0', '\xe1', '\xe2', '\xe3', '\xe4', '\xe5', '\xe6', '\xe7', '\xe8', '\xe9', '\xea', '\xeb', '\xec', '\xed', '\xee', '\xef',
        '\xf0', '\xf1', '\xf2', '\xf3', '\xf4', '\xf5', '\xf6', '\xf7', '\xf8', '\xf9', '\xfa', '\xfb', '\xfc', '\xfd', '\xfe', '\xff',
    });
    const Char8 = OneOf([_]u8{
        '\x01', '\x02', '\x03', '\x04', '\x05', '\x06', '\x07', '\x08', '\x09', '\x0a', '\x0b', '\x0c', '\x0d', '\x0e', '\x0f',
        '\x10', '\x11', '\x12', '\x13', '\x14', '\x15', '\x16', '\x17', '\x18', '\x19', '\x1a', '\x1b', '\x1c', '\x1d', '\x1e',
        '\x1f', '\x20', '\x21', '\x22', '\x23', '\x24', '\x25', '\x26', '\x27', '\x28', '\x29', '\x2a', '\x2b', '\x2c', '\x2d',
        '\x2e', '\x2f', '\x30', '\x31', '\x32', '\x33', '\x34', '\x35', '\x36', '\x37', '\x38', '\x39', '\x3a', '\x3b', '\x3c',
        '\x3d', '\x3e', '\x3f', '\x40', '\x41', '\x42', '\x43', '\x44', '\x45', '\x46', '\x47', '\x48', '\x49', '\x4a', '\x4b',
        '\x4c', '\x4d', '\x4e', '\x4f', '\x50', '\x51', '\x52', '\x53', '\x54', '\x55', '\x56', '\x57', '\x58', '\x59', '\x5a',
        '\x5b', '\x5c', '\x5d', '\x5e', '\x5f', '\x60', '\x61', '\x62', '\x63', '\x64', '\x65', '\x66', '\x67', '\x68', '\x69',
        '\x6a', '\x6b', '\x6c', '\x6d', '\x6e', '\x6f', '\x70', '\x71', '\x72', '\x73', '\x74', '\x75', '\x76', '\x77', '\x78',
        '\x79', '\x7a', '\x7b', '\x7c', '\x7d', '\x7e', '\x7f', '\x80', '\x81', '\x82', '\x83', '\x84', '\x85', '\x86', '\x87',
        '\x88', '\x89', '\x8a', '\x8b', '\x8c', '\x8d', '\x8e', '\x8f', '\x90', '\x91', '\x92', '\x93', '\x94', '\x95', '\x96',
        '\x97', '\x98', '\x99', '\x9a', '\x9b', '\x9c', '\x9d', '\x9e', '\x9f', '\xa0', '\xa1', '\xa2', '\xa3', '\xa4', '\xa5',
        '\xa6', '\xa7', '\xa8', '\xa9', '\xaa', '\xab', '\xac', '\xad', '\xae', '\xaf', '\xb0', '\xb1', '\xb2', '\xb3', '\xb4',
        '\xb5', '\xb6', '\xb7', '\xb8', '\xb9', '\xba', '\xbb', '\xbc', '\xbd', '\xbe', '\xbf', '\xc0', '\xc1', '\xc2', '\xc3',
        '\xc4', '\xc5', '\xc6', '\xc7', '\xc8', '\xc9', '\xca', '\xcb', '\xcc', '\xcd', '\xce', '\xcf', '\xd0', '\xd1', '\xd2',
        '\xd3', '\xd4', '\xd5', '\xd6', '\xd7', '\xd8', '\xd9', '\xda', '\xdb', '\xdc', '\xdd', '\xde', '\xdf', '\xe0', '\xe1',
        '\xe2', '\xe3', '\xe4', '\xe5', '\xe6', '\xe7', '\xe8', '\xe9', '\xea', '\xeb', '\xec', '\xed', '\xee', '\xef', '\xf0',
        '\xf1', '\xf2', '\xf3', '\xf4', '\xf5', '\xf6', '\xf7', '\xf8', '\xf9', '\xfa', '\xfb', '\xfc', '\xfd', '\xfe', '\xff',
    });
    const LParen = SingleVal("(");
    const RParen = SingleVal(")");
    const LBrace = SingleVal("{");
    const RBrace = SingleVal("}");
    const LBracket = SingleVal("[");
    const RBraceet = SingleVal("]");
    const Plus = SingleVal("+");
    const Address = And(.{
        .{ "l_paren", LParen },
        .{ "addr_name", AddrName },
        .{ "space", Space },
        .{ "addr_adl", AddrAdl },
        .{ "space", Space },
        .{ "addr_mailbox", AddrMailBox },
        .{ "r_paren", RParen },
    });
    const AddrName = NString;
    const AddrAdl = NString;
    const AddrMailBox = NString;
    const NString = Or(.{ .{ "string", String }, .{ "nil", Nil } });
    const Nil = SingleVal("NIL");
    const String = Or(.{ .{ "quoted", Quoted }, .{ "literal", Literal } });
    const Quoted = And(.{
        DQuote,
        ZeroOrMore(QuotedChar),
        DQuote,
    });
    const Number64 = OneOrMore(Digit);
    const Literal = And(.{
        .{ "l_brace", LBrace },
        .{ "num", Number64 },
        .{ "maybe_plus", Optional(Plus) },
        .{ "r_brace", RBrace },
        .{ "crlf", CrLf },
        .{ "msg", ZeroOrMore(Char8) },
    });
    const QuotedChar = void;
    const Command = And(
        .{ "tag", Tag },
        .{ "sp", Space },
        .{ "command", OneOf(&[_]type{ CommandAny, CommandAuth, CommandNonAuth, CommandSelect }) },
        .{ "crlf", CrLf },
    );
    const Capability = SingleVal("CAPABILITY");
    const Logout = SingleVal("LOGOUT");
    const Noop = SingleVal("NOOP");
    const CommandAny = OneOf(&[_]type{ Capability, Logout, Noop });
    const CommandAuth = void;
    const CommandNonAuth = void;
    const CommandSelect = void;
    const Tag = void;
};

fn SingleVal(comptime val: anytype) type {
    return struct {
        const ChildTy = @TypeOf(val);
        const Val = val;
    };
}

fn ZeroOrMore(comptime T: type) type {
    return struct { rest: []T };
}

fn OneOrMore(comptime T: type) type {
    return struct { first: T, rest: []T };
}

fn OneOf(comptime T: []const type) type {
    const Type = std.builtin.Type;
    const Union = Type.Union;
    const Decl = Type.Declaration;
    const Field = Type.UnionField;
    var Fields: [T.len]Field = undefined;
    inline for (T) |ty, i| {
        Fields[i] = Field{
            .name = @typeName(ty),
            .field_type = ty,
            .alignment = @alignOf(ty[1]),
        };
    }
    return @Type(Type{ .Union = Union{
        .layout = .Auto,
        .tag_type = null,
        .fields = &Fields,
        .decls = &[_]Decl{},
    } });
}

fn Or(comptime T: anytype) type {
    const Type = std.builtin.Type;
    const Union = Type.Union;
    const Decl = Type.Declaration;
    const Field = Type.UnionField;
    var Fields: [T.len]Field = undefined;
    inline for (T) |ty, i| {
        Fields[i] = Field{
            .name = ty[0],
            .field_type = ty[1],
            .alignment = @alignOf(ty[1]),
        };
    }
    return @Type(Type{ .Union = Union{
        .layout = .Auto,
        .tag_type = null,
        .fields = &Fields,
        .decls = &[_]Decl{},
    } });
}

fn And(comptime T: anytype) type {
    const Type = std.builtin.Type;
    const Struct = Type.Struct;
    const Decl = Type.Declaration;
    const Field = Type.StructField;
    var Fields: [T.len]Field = undefined;
    inline for (T) |ty, i| {
        Fields[i] = Field{
            .name = ty[0],
            .field_type = ty[1],
            .default_value = null,
            .is_comptime = false,
            .alignment = @alignOf(ty[1]),
        };
    }
    return @Type(Type{ .Struct = Struct{
        .layout = .Auto,
        .fields = &Fields,
        .decls = &[_]Decl{},
        .is_tuple = false,
    } });
}

fn Optional(comptime T: anytype) type {
    return struct {
        val: ?T,
    };
}

test "or" {
    const Slice = []const u8;
    const Int = u32;
    const Union = union { slice: Slice, int: Int };
    const OrUnion = Or(.{ .{ "slice", Slice }, .{ "int", Int } });

    const UnionInfo = @typeInfo(Union).Union;
    const OrUnionInfo = @typeInfo(OrUnion).Union;

    try std.testing.expect(UnionInfo.layout == OrUnionInfo.layout);
    {
        const UnionFields = UnionInfo.fields;
        const OrUnionFields = OrUnionInfo.fields;
        inline for (UnionFields) |union_field, i| {
            const or_union_field = OrUnionFields[i];
            try std.testing.expectEqualStrings(union_field.name, or_union_field.name);
            try std.testing.expect(union_field.field_type == or_union_field.field_type);
            try std.testing.expect(union_field.alignment == or_union_field.alignment);
        }
    }
    try std.testing.expectEqualSlices(std.builtin.Type.Declaration, UnionInfo.decls, OrUnionInfo.decls);
}

test "and" {
    const Slice = []const u8;
    const Int = u32;
    const Struct = struct { slice: Slice, int: Int };
    const AndStruct = And(.{ .{ "slice", Slice }, .{ "int", Int } });

    const StructInfo = @typeInfo(Struct).Struct;
    const AndStructInfo = @typeInfo(AndStruct).Struct;

    try std.testing.expect(StructInfo.layout == AndStructInfo.layout);
    {
        const StructFields = StructInfo.fields;
        const AndStructFields = AndStructInfo.fields;
        inline for (StructFields) |struct_field, i| {
            const or_struct_field = AndStructFields[i];
            try std.testing.expectEqualStrings(struct_field.name, or_struct_field.name);
            try std.testing.expect(struct_field.field_type == or_struct_field.field_type);
            try std.testing.expect(struct_field.alignment == or_struct_field.alignment);
        }
    }
    try std.testing.expectEqualSlices(std.builtin.Type.Declaration, StructInfo.decls, AndStructInfo.decls);
}

test "parse connect" {
    const login_str = "a001 LOGIN SMITH SESAME";
    const login = Definitions.Command.parse(login_str);
    _ = login;
}
