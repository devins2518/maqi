pub struct Parser<'a> {
    backing: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(backing: &'a str) -> Self {
        Self { backing }
    }
}

enum Types {
    SP,
    CTL,
    CRLF,
    Alpha,
    Digit,
    DQuote,
    Octet,
    Address,
    AddrAdl,
    AddrHost,
    AddrMailbox,
    AddrName,
    Append,
    AppendUID,
    AString,
    AStringChar,
    Atom,
    AtomChar,
    AtomSpecials,
    Authenticate,
    AuthType,
    Base64,
    Base64Char,
    Base64Terminal,
    Body,
    BodyExtension,
    BodyExt1part,
    BodyExtMpart,
    BodyFields,
    BodyFldDesc,
    BodyFldDsp,
    BodyFldEnc,
    BodyFldId,
    BodyFldLang,
    BodyFldLoc,
    BodyFldLines,
    BodyFldMd5,
    BodyFldOctets,
    BodyFldParam,
    BodyType1part,
    BodyTypeBasic,
    BodyTypeMpart,
    BodyTypeMsg,
    BodyTypeText,
    Capability,
    CapabilityData,
    Char,
    Char8,
    Charset,
    ChildinfoExtendedItem,
    ChildMboxFlag,
    Command,
    CommandAny,
    CommandAuth,
    CommandNonauth,
    CommandSelect,
    ContinueReq,
    Copy,
    Create,
    Date,
    DateDay,
    DateDayFixed,
    DateMonth,
    DateText,
    DateYear,
    DateTime,
    Delete,
    DigitNz,
    EItemStandardTag,
    EItemVendorTag,
    Enable,
    EnableData,
    Envelope,
    EnvBcc,
    EnvCc,
    EnvDate,
    EnvFrom,
    EnvInReplyTo,
    EnvMessageId,
    EnvReplyTo,
    EnvSender,
    EnvSubject,
    EnvTo,
    EsearchResponse,
    Examine,
    Fetch,
    FetchAtt,
    Flag,
    FlagExtension,
    FlagFetch,
    FlagKeyword,
    FlagList,
    FlagPerm,
    Greeting,
    HeaderFldName,
    HeaderList,
    Idle,
    InitialResp,
    List,
    ListMailbox,
    ListChar,
    ListReturnOpt,
    ListReturnOpts,
    ListSelectBaseOpt,
    ListSelectBaseOptQuoted,
    ListSelectIndependentOpt,
    ListSelectModOpt,
    ListSelectOpt,
    ListSelectOpts,
    ListWildcards,
    Literal,
    Literal8,
    Login,
    Mailbox,
    MailboxData,
    MailboxList,
    MboxListExtended,
    MboxListExtendedItem,
    MboxListExtendedItemTag,
    MboxOrPat,
    MbxListFlags,
    MbxListOflag,
    MbxListSflag,
    MediaBasic,
    MediaMessage,
    MediaSubtype,
    MediaText,
    MessageData,
    Move,
    MsgAtt,
    MsgAttDynamic,
    MsgAttStatic,
    NameComponent,
    Namespace,
    NamespaceCommand,
    NamespaceDescr,
    NamespaceResponseExtensions,
    NamespaceResponseExtension,
    NamespaceResponse,
    Nil,
    Nstring,
    Number,
    Number64,
    NzNumber,
    NzNumber64,
    ObsoleteFlagRecent,
    ObsoleteRecentResponse,
    ObsoleteSearchResponse,
    OldnameExtendedItem,
    OptionExtension,
    OptionStandardTag,
    OptionValComp,
    OptionValue,
    OptionVendorTag,
    PartialRange,
    Partial,
    Password,
    Patterns,
    Quoted,
    QUOTEDCHAR,
    QuotedSpecials,
    Rename,
    Response,
    ResponseData,
    ResponseDone,
    ResponseFatal,
    ResponseTagged,
    RespCodeApnd,
    RespCodeCopy,
    RespCondAuth,
    RespCondBye,
    RespCondState,
    RespSpecials,
    RespText,
    RespTextCode,
    ReturnOption,
    Search,
    SearchCorrelator,
    SearchKey,
    SearchModifierName,
    SearchModParams,
    SearchProgram,
    SearchRetDataExt,
    SearchReturnData,
    SearchReturnOpts,
    SearchReturnOpt,
    SearchRetOptExt,
    SearchReturnValue,
    Section,
    SectionBinary,
    SectionMsgtext,
    SectionPart,
    SectionSpec,
    SectionText,
    Select,
    SeqNumber,
    SeqRange,
    SequenceSet,
    SeqLastCommand,
    Status,
    StatusAtt,
    StatusAttVal,
    StatusAttList,
    StatusOption,
    Store,
    StoreAttFlags,
    String,
    Subscribe,
    Tag,
    TagString,
    TaggedExtLabel,
    TaggedLabelFchar,
    TaggedLabelChar,
    TaggedExtComp,
    TaggedExtSimple,
    TaggedExtVal,
    Text,
    TEXTCHAR,
    Time,
    UID,
    UIDExpunge,
    UIDSet,
    UIDRange,
    Uniqueid,
    Unsubscribe,
    Userid,
    Utf8Char,
    UTF8_2,
    UTF8_3,
    UTF8_4,
    VendorToken,
    Zone,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    // Single or multi character tokens
    DOLLAR,       // $
    LPAREN,       // (
    RPAREN,       // )
    LBRACKET,     // [
    RBRACKET,     // ]
    LBRACE,       // {
    RBRACE,       // }
    LANGLE,       // <
    RANGLE,       // >
    PLUS,         // +
    FWSLASH,      // /
    BWSLASH,      // \
    EQUAL,        // =
    DBLEQUAL,     // ==
    HYPHEN,       // -
    BWSLASHSTAR,  // \*
    STAR,         // *
    PERCENT,      // %
    APPROXLBRACE, // ~{
    PERIOD,       // .
    COLON,        // :
    COMMA,        // ,
    UNDERSCORE,   // _
    CRLF,         // \r\n
    DQUOTE,       // "
    SP,

    // Word tokens
    Alert,                // ALERT
    All,                  // ALL
    AlreadyExists,        // ALREADYEXISTS
    Answered,             // ANSWERED
    Append,               // APPEND
    AppendUID,            // APPENDUID
    Application,          // APPLICATION
    Apr,                  // Apr
    Audio,                // AUDIO
    Aug,                  // Aug
    AuthEq(String),       // AUTH=
    Authenticate,         // AUTHENTICATE
    AuthenticationFailed, // AUTHENTICATIONFAILED
    AuthorizationFailed,  // AUTHORIZATIONFAILED
    BWSlashAnswered,      // \Answered
    BWSlashDeleted,       // \Deleted
    BWSlashDraft,         // \Draft
    BWSlashFlagged,       // \Flagged
    BWSlashHasChildren,   // \HasChildren
    BWSlashHasNoChildren, // \HasNoChildren
    BWSlashMarked,        // \Marked
    BWSlashNoInferiors,   // \Noinferiors
    BWSlashNoSelect,      // \Noselect
    BWSlashNonExistent,   // \NonExistent
    BWSlashRecent,        // \Recent
    BWSlashRemote,        // \Remote
    BWSlashSeen,          // \Seen
    BWSlashSubscribed,    // \Subscribed
    BWSlashUnmarked,      // \Unmarked
    Bad,                  // BAD
    BadCharset,           // BADCHARSET
    Base64,               // BASE64
    Bcc,                  // BCC
    Before,               // BEFORE
    Binary,               // BINARY
    BinaryDotSize,        // BINARY.SIZE
    Body,                 // BODY
    BodyDotPeek,          // BODY.PEEK
    Bye,                  // BYE
    Cannot,               // CANNOT
    Capability,           // CAPABILITY
    Cc,                   // CC
    Charset,              // CHARSET
    ChildInfo,            // CHILDINFO
    Children,             // CHILDREN
    ClientBug,            // CLIENTBUG
    Close,                // CLOSE
    Closed,               // CLOSED
    ContactAdmin,         // CONTACTADMIN
    Copy,                 // COPY
    CopyUID,              // COPYUID
    Corruption,           // CORRUPTION
    Count,                // COUNT
    Create,               // CREATE
    Dec,                  // Dec
    Delete,               // DELETE
    Deleted,              // DELETED
    DollarForwarded,      // $Forwarded
    DollarJunk,           // $Junk
    DollarMDNSent,        // $MDNSent
    DollarNotJunk,        // $NotJunk
    DollarPhishing,       // $Phishing
    Done,                 // DONE
    DotNot,               // .NOT
    DotPeek,              // .PEEK
    DotSilent,            // .SILENT
    Draft,                // DRAFT
    ESearch,              // ESEARCH
    EightBit,             // 8BIT
    Enable,               // ENABLE
    Enabled,              // ENABLED
    Envelope,             // ENVELOPE
    Examine,              // EXAMINE
    Exists,               // EXISTS
    Expired,              // EXPIRED
    Expunge,              // EXPUNGE
    ExpungeIssued,        // EXPUNGEISSUED
    Fast,                 // FAST
    Feb,                  // Feb
    Fetch,                // FETCH
    Flagged,              // FLAGGED
    Flags,                // FLAGS
    Font,                 // FONT
    From,                 // FROM
    Full,                 // FULL
    Global,               // GLOBAL
    HasChildren,          // HASCHILDREN
    Header,               // HEADER
    HeaderDotFields,      // HEADER.FIELDS
    IMAP4Rev1,            // IMAP4rev1
    IMAP4Rev2,            // IMAP4rev2
    Idle,                 // IDLE
    Image,                // IMAGE
    InUse,                // INUSE
    Inbox,                // INBOX
    Internaldate,         // INTERNALDATE
    Jan,                  // Jan
    Jul,                  // Jul
    Jun,                  // Jun
    Keyword,              // KEYWORD
    Larger,               // LARGER
    Limit,                // LIMIT
    List,                 // LIST
    Login,                // LOGIN
    Logout,               // LOGOUT
    Mar,                  // Mar
    Max,                  // MAX
    May,                  // May
    Message,              // MESSAGE
    Messages,             // MESSAGES
    Mime,                 // MIME
    Min,                  // MIN
    Model,                // MODEL
    Move,                 // MOVE
    Namespace,            // NAMESPACE
    Nil,                  // NIL
    No,                   // NO
    NoPerm,               // NOPERM
    NonExistent,          // NONEXISTENT
    Noop,                 // NOOP
    NotSaved,             // NOTSAVED
    Nov,                  // Nov
    Oct,                  // Oct
    Ok,                   // OK
    OldName,              // OLDNAME
    On,                   // ON
    Or,                   // OR
    OverQuota,            // OVERQUOTA
    Parse,                // PARSE
    PermanentFlags,       // PERMANENTFLAGS
    PreAuth,              // PREAUTH
    PrivacyRequired,      // PRIVACYREQUIRED
    QuotedPrintable,      // QUOTE-PRINTABLE
    RFC822,               // RFC822
    RFC822DotSize,        // RFC822.SIZE
    ReadHyphenOnly,       // READ-ONLY
    ReadHyphenWrite,      // READ-WRITE
    RecursiveMatch,       // RECURSIVEMATCH
    Remote,               // REMOTE
    Return,               // RETURN
    Save,                 // SAVE
    Search,               // SEARCH
    Seen,                 // SEEN
    Select,               // SELECT
    SentBefore,           // SENTBEFORE
    SentOn,               // SENTON
    SentSince,            // SENTSINCE
    Sep,                  // Sep
    ServerBug,            // SERVERBUG
    SevenBit,             // 7BIT
    Since,                // SINCE
    Size,                 // SIZE
    Smaller,              // SMALLER
    StartTLS,             // STARTTLS
    Status,               // STATUS
    Store,                // STORE
    Structure,            // STRUCTURE
    Subject,              // SUBJECT
    Subscribed,           // SUBSCRIBED
    Tag,                  // TAG
    Text,                 // TEXT
    To,                   // TO
    TryCreate,            // TRYCREATE
    UID,                  // UID
    UIDNext,              // UIDNEXT
    UIDNotSticky,         // UIDNOTSTICKY
    UIDValidity,          // UIDVALIDITY
    Unanswered,           // UNANSWERED
    Unavailable,          // UNAVAILABLE
    Undeleted,            // UNDELETED
    Undraft,              // UNDRAFT
    Unflagged,            // UNFLAGGED
    Unkeyword,            // UNKEYWORD
    UnknownHyphenCTE,     // UNKNOWN-CTE
    Unseen,               // UNSEEN
    Unselect,             // UNSELECT
    Unsubscribe,          // UNSUBSCRIBE
    VenderDot,            // vender.
    Video,                // VIDEO
    Other(String),        // Unscanned
}
