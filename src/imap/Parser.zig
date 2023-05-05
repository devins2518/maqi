const std = @import("std");
const parzer = @import("parzer");

const ParseError = error{DidNotFind};

const Definitions = struct {
    pub const Space = parzer.SingleValue(' ');
    // pub const Control = mecha.ascii.cntrl;
    pub const CrLf = parzer.SingleValue("\r\n");
    // pub const Alpha = mecha.ascii.alpha;
    // pub const Digit = mecha.ascii.digit(10);
    pub const DQuote = parzer.SingleValue('"');
};
