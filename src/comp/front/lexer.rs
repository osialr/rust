import std.io;
import std._str;
import std._vec;
import std._int;
import std.map;
import std.map.hashmap;
import std.option;
import std.option.some;
import std.option.none;
import util.common;
import util.common.new_str_hash;

state type reader = state obj {
    fn is_eof() -> bool;
    fn curr() -> char;
    fn next() -> char;
    fn init();
    fn bump();
    fn mark();
    fn get_mark_chpos() -> uint;
    fn get_chpos() -> uint;
    fn get_keywords() -> hashmap[str,token.token];
    fn get_reserved() -> hashmap[str,()];
    fn get_filemap() -> codemap.filemap;
};

fn new_reader(io.reader rdr, str filename, codemap.filemap filemap)
    -> reader {
    state obj reader(str file,
                     uint len,
                     mutable uint pos,
                     mutable char ch,
                     mutable uint mark_chpos,
                     mutable uint chpos,
                     hashmap[str,token.token] keywords,
                     hashmap[str,()] reserved,
                     codemap.filemap fm) {

        fn is_eof() -> bool {
            ret ch == -1 as char;
        }

        fn mark() { mark_chpos = chpos; }
        fn get_mark_chpos() -> uint { ret mark_chpos; }
        fn get_chpos() -> uint { ret chpos; }

        fn curr() -> char {
            ret ch;
        }

        fn next() -> char {
            if (pos < len) {ret _str.char_at(file, pos);}
            else {ret -1 as char;}
        }

        fn init() {
            if (pos < len) {
                auto next = _str.char_range_at(file, pos);
                pos = next._1;
                ch = next._0;
            }
        }

        fn bump() {
            if (pos < len) {
                chpos += 1u;
                if (ch == '\n') {
                    codemap.next_line(fm, chpos);
                }
                auto next = _str.char_range_at(file, pos);
                pos = next._1;
                ch = next._0;
            } else {
                ch = -1 as char;
            }
        }

        fn get_keywords() -> hashmap[str,token.token] {
            ret keywords;
        }

        fn get_reserved() -> hashmap[str,()] {
            ret reserved;
        }

        fn get_filemap() -> codemap.filemap {
            ret fm;
        }
    }
    auto file = _str.unsafe_from_bytes(rdr.read_whole_stream());
    auto rd = reader(file, _str.byte_len(file), 0u, -1 as char,
                     filemap.start_pos, filemap.start_pos,
                     keyword_table(),
                     reserved_word_table(),
                     filemap);
    rd.init();
    ret rd;
}

fn keyword_table() -> std.map.hashmap[str, token.token] {
    auto keywords = new_str_hash[token.token]();

    keywords.insert("mod", token.MOD);
    keywords.insert("use", token.USE);
    keywords.insert("meta", token.META);
    keywords.insert("auth", token.AUTH);

    keywords.insert("syntax", token.SYNTAX);

    keywords.insert("if", token.IF);
    keywords.insert("else", token.ELSE);
    keywords.insert("while", token.WHILE);
    keywords.insert("do", token.DO);
    keywords.insert("alt", token.ALT);
    keywords.insert("case", token.CASE);

    keywords.insert("for", token.FOR);
    keywords.insert("each", token.EACH);
    keywords.insert("break", token.BREAK);
    keywords.insert("cont", token.CONT);
    keywords.insert("put", token.PUT);
    keywords.insert("ret", token.RET);
    keywords.insert("be", token.BE);

    keywords.insert("fail", token.FAIL);
    keywords.insert("drop", token.DROP);

    keywords.insert("type", token.TYPE);
    keywords.insert("check", token.CHECK);
    keywords.insert("claim", token.CLAIM);
    keywords.insert("prove", token.PROVE);

    keywords.insert("abs", token.ABS);

    keywords.insert("state", token.STATE);
    keywords.insert("gc", token.GC);

    keywords.insert("unsafe", token.UNSAFE);

    keywords.insert("native", token.NATIVE);
    keywords.insert("mutable", token.MUTABLE);
    keywords.insert("auto", token.AUTO);

    keywords.insert("fn", token.FN);
    keywords.insert("iter", token.ITER);

    keywords.insert("import", token.IMPORT);
    keywords.insert("export", token.EXPORT);

    keywords.insert("let", token.LET);
    keywords.insert("const", token.CONST);

    keywords.insert("log", token.LOG);
    keywords.insert("log_err", token.LOG_ERR);
    keywords.insert("spawn", token.SPAWN);
    keywords.insert("thread", token.THREAD);
    keywords.insert("yield", token.YIELD);
    keywords.insert("join", token.JOIN);

    keywords.insert("bool", token.BOOL);

    keywords.insert("int", token.INT);
    keywords.insert("uint", token.UINT);
    keywords.insert("float", token.FLOAT);

    keywords.insert("char", token.CHAR);
    keywords.insert("str", token.STR);


    keywords.insert("rec", token.REC);
    keywords.insert("tup", token.TUP);
    keywords.insert("tag", token.TAG);
    keywords.insert("vec", token.VEC);
    keywords.insert("any", token.ANY);

    keywords.insert("obj", token.OBJ);
    keywords.insert("self", token.SELF);

    keywords.insert("port", token.PORT);
    keywords.insert("chan", token.CHAN);

    keywords.insert("task", token.TASK);

    keywords.insert("true", token.LIT_BOOL(true));
    keywords.insert("false", token.LIT_BOOL(false));

    keywords.insert("in", token.IN);

    keywords.insert("as", token.AS);
    keywords.insert("with", token.WITH);

    keywords.insert("bind", token.BIND);

    keywords.insert("u8", token.MACH(common.ty_u8));
    keywords.insert("u16", token.MACH(common.ty_u16));
    keywords.insert("u32", token.MACH(common.ty_u32));
    keywords.insert("u64", token.MACH(common.ty_u64));
    keywords.insert("i8", token.MACH(common.ty_i8));
    keywords.insert("i16", token.MACH(common.ty_i16));
    keywords.insert("i32", token.MACH(common.ty_i32));
    keywords.insert("i64", token.MACH(common.ty_i64));
    keywords.insert("f32", token.MACH(common.ty_f32));
    keywords.insert("f64", token.MACH(common.ty_f64));

    ret keywords;
}

fn reserved_word_table() -> std.map.hashmap[str, ()] {
    auto reserved = new_str_hash[()]();
    reserved.insert("f16", ());  // IEEE 754-2008 'binary16' interchange fmt
    reserved.insert("f80", ());  // IEEE 754-1985 'extended'
    reserved.insert("f128", ()); // IEEE 754-2008 'binary128'
    reserved.insert("m32", ());  // IEEE 754-2008 'decimal32'
    reserved.insert("m64", ());  // IEEE 754-2008 'decimal64'
    reserved.insert("m128", ()); // IEEE 754-2008 'decimal128'
    reserved.insert("dec", ());  // One of m32, m64, m128
    ret reserved;
}

fn in_range(char c, char lo, char hi) -> bool {
    ret lo <= c && c <= hi;
}

fn is_alpha(char c) -> bool {
    ret in_range(c, 'a', 'z') ||
        in_range(c, 'A', 'Z');
}

fn is_dec_digit(char c) -> bool {
    ret in_range(c, '0', '9');
}

fn is_alnum(char c) -> bool {
    ret is_alpha(c) || is_dec_digit(c);
}

fn is_hex_digit(char c) -> bool {
    ret in_range(c, '0', '9') ||
        in_range(c, 'a', 'f') ||
        in_range(c, 'A', 'F');
}

fn is_bin_digit(char c) -> bool {
    ret c == '0' || c == '1';
}

fn dec_digit_val(char c) -> int {
    ret (c as int) - ('0' as int);
}

fn hex_digit_val(char c) -> int {
    if (in_range(c, '0', '9')) {
        ret (c as int) - ('0' as int);
    }

    if (in_range(c, 'a', 'f')) {
        ret ((c as int) - ('a' as int)) + 10;
    }

    if (in_range(c, 'A', 'F')) {
        ret ((c as int) - ('A' as int)) + 10;
    }

    fail;
}

fn bin_digit_value(char c) -> int {
    if (c == '0') { ret 0; }
    ret 1;
}

fn is_whitespace(char c) -> bool {
    ret c == ' ' || c == '\t' || c == '\r' || c == '\n';
}

fn consume_any_whitespace(reader rdr) {
    while (is_whitespace(rdr.curr())) {
        rdr.bump();
    }
    be consume_any_line_comment(rdr);
}

fn consume_any_line_comment(reader rdr) {
    if (rdr.curr() == '/') {
        alt (rdr.next()) {
            case ('/') {
                while (rdr.curr() != '\n' && !rdr.is_eof()) {
                    rdr.bump();
                }
                // Restart whitespace munch.
                be consume_any_whitespace(rdr);
            }
            case ('*') {
                rdr.bump();
                rdr.bump();
                be consume_block_comment(rdr);
            }
            case (_) {
                ret;
            }
        }
    }
}


fn consume_block_comment(reader rdr) {
    let int level = 1;
    while (level > 0) {
        if (rdr.curr() == '/' && rdr.next() == '*') {
            rdr.bump();
            rdr.bump();
            level += 1;
        } else {
            if (rdr.curr() == '*' && rdr.next() == '/') {
                rdr.bump();
                rdr.bump();
                level -= 1;
            } else {
                rdr.bump();
            }
        }
        if (rdr.is_eof()) {
            log_err "unterminated block comment";
            fail;
        }
    }
    // restart whitespace munch.
    be consume_any_whitespace(rdr);
}

fn digits_to_string(str s) -> int {

    let int accum_int = 0;
    let int i = 0;

    for (u8 c in s) {
        accum_int *= 10;
        accum_int += dec_digit_val(c as char);
    }

    ret accum_int;
}

fn scan_exponent(reader rdr) -> option.t[str] {
    auto c = rdr.curr();
    auto res = "";

    if (c == 'e' || c == 'E') {
        res += _str.from_bytes(vec(c as u8));
        rdr.bump();
        c = rdr.curr();
        if (c == '-' || c == '+') {
            res += _str.from_bytes(vec(c as u8));
            rdr.bump();
        }
        auto exponent = scan_dec_digits(rdr);
        if (_str.byte_len(exponent) > 0u) {
            ret(some(res + exponent));
        }
        else {
            log_err ("scan_exponent: bad fp literal");
            fail;
        }
    }
    else {
        ret none[str];
    }
}

fn scan_dec_digits(reader rdr) -> str {

    auto c = rdr.curr();
    let str res = "";

    while (is_dec_digit (c) || c == '_') {
        if (c != '_') {
            res += _str.from_bytes(vec(c as u8));
        }
        rdr.bump();
        c = rdr.curr();
    }

    ret res;
}

fn scan_number(mutable char c, reader rdr) -> token.token {
    auto accum_int = 0;
    let str dec_str = "";
    let bool is_dec_integer = false;
    auto n = rdr.next();

    if (c == '0' && n == 'x') {
        rdr.bump();
        rdr.bump();
        c = rdr.curr();
        while (is_hex_digit(c) || c == '_') {
            if (c != '_') {
                accum_int *= 16;
                accum_int += hex_digit_val(c);
            }
            rdr.bump();
            c = rdr.curr();
        }
    } else if (c == '0' && n == 'b') {
        rdr.bump();
        rdr.bump();
        c = rdr.curr();
        while (is_bin_digit(c) || c == '_') {
            if (c != '_') {
                accum_int *= 2;
                accum_int += bin_digit_value(c);
            }
            rdr.bump();
            c = rdr.curr();
        }
    } else {
        dec_str = scan_dec_digits(rdr);
        is_dec_integer = true;
    }

    if (is_dec_integer) {
        accum_int = digits_to_string(dec_str);
    }

    c = rdr.curr();
    n = rdr.next();

    if (c == 'u' || c == 'i') {
        let bool signed = (c == 'i');
        rdr.bump();
        c = rdr.curr();
        if (c == '8') {
            rdr.bump();
            if (signed) {
                ret token.LIT_MACH_INT(common.ty_i8, accum_int);
            } else {
                ret token.LIT_MACH_INT(common.ty_u8, accum_int);
            }
        }

        n = rdr.next();
        if (c == '1' && n == '6') {
            rdr.bump();
            rdr.bump();
            if (signed) {
                ret token.LIT_MACH_INT(common.ty_i16, accum_int);
            } else {
                ret token.LIT_MACH_INT(common.ty_u16, accum_int);
            }
        }
        if (c == '3' && n == '2') {
            rdr.bump();
            rdr.bump();
            if (signed) {
                ret token.LIT_MACH_INT(common.ty_i32, accum_int);
            } else {
                ret token.LIT_MACH_INT(common.ty_u32, accum_int);
            }
        }

        if (c == '6' && n == '4') {
            rdr.bump();
            rdr.bump();
            if (signed) {
                ret token.LIT_MACH_INT(common.ty_i64, accum_int);
            } else {
                ret token.LIT_MACH_INT(common.ty_u64, accum_int);
            }
        }

        if (signed) {
            ret token.LIT_INT(accum_int);
        } else {
            // FIXME: should cast in the target bit-width.
            ret token.LIT_UINT(accum_int as uint);
        }
    }
    c = rdr.curr();

    if (c == '.') {
        // Parse a floating-point number.
        rdr.bump();
        auto dec_part = scan_dec_digits(rdr);
        auto float_str = dec_str + "." + dec_part;
        c = rdr.curr();
        auto exponent_str = scan_exponent(rdr);
        alt (exponent_str) {
            case (some[str](?s)) {
                float_str += s;
            }
            case (none[str]) {
            }
        }

        c = rdr.curr();
        if (c == 'f') {
            rdr.bump();
            c = rdr.curr();
            n = rdr.next();
            if (c == '3' && n == '2') {
                rdr.bump(); rdr.bump();
                ret token.LIT_MACH_FLOAT(util.common.ty_f32,
                                         float_str);
            }
            else if (c == '6' && n == '4') {
                rdr.bump(); rdr.bump();
                ret token.LIT_MACH_FLOAT(util.common.ty_f64,
                                         float_str);
                /* FIXME: if this is out of range for either a 32-bit or
                   64-bit float, it won't be noticed till the back-end */
            }
        }
        else {
            ret token.LIT_FLOAT(float_str);
        }
    }

    auto maybe_exponent = scan_exponent(rdr);
    alt(maybe_exponent) {
        case(some[str](?s)) {
            ret token.LIT_FLOAT(dec_str + s);
        }
        case(none[str]) {
                ret token.LIT_INT(accum_int);
        }
    }
}

fn scan_numeric_escape(reader rdr) -> char {

    auto n_hex_digits = 0;

    check (rdr.curr() == '\\');

    alt (rdr.next()) {
        case ('x') { n_hex_digits = 2; }
        case ('u') { n_hex_digits = 4; }
        case ('U') { n_hex_digits = 8; }
        case (?c) {
            log_err #fmt("unknown numeric character escape: %d", c as int);
            fail;
        }
    }

    rdr.bump(); // advance curr past \

    auto n = rdr.next();
    auto accum_int = 0;

    while (n_hex_digits != 0) {
        if (!is_hex_digit(n)) {
            log_err #fmt("illegal numeric character escape: %d", n as int);
            fail;
        }
        accum_int *= 16;
        accum_int += hex_digit_val(n);
        rdr.bump();
        n = rdr.next();
        n_hex_digits -= 1;
    }
    ret accum_int as char;
}


fn next_token(reader rdr) -> token.token {
    auto accum_str = "";

    consume_any_whitespace(rdr);

    if (rdr.is_eof()) { ret token.EOF; }

    rdr.mark();
    auto c = rdr.curr();

    if (is_alpha(c) || c == '_') {
        while (is_alnum(c) || c == '_') {
            _str.push_char(accum_str, c);
            rdr.bump();
            c = rdr.curr();
        }

        if (_str.eq(accum_str, "_")) {
            ret token.UNDERSCORE;
        }

        auto kwds = rdr.get_keywords();
        if (kwds.contains_key(accum_str)) {
            ret kwds.get(accum_str);
        }

        auto rsvd = rdr.get_reserved();
        if (rsvd.contains_key(accum_str)) {
            log_err #fmt("reserved keyword: %s", accum_str);
            fail;
        }

        ret token.IDENT(accum_str);
    }

    if (is_dec_digit(c)) {
        ret scan_number(c, rdr);
    }

    fn binop(reader rdr, token.binop op) -> token.token {
        rdr.bump();
        if (rdr.curr() == '=') {
            rdr.bump();
            ret token.BINOPEQ(op);
        } else {
            ret token.BINOP(op);
        }
    }

    alt (c) {
        // One-byte tokens.
        case (':') { rdr.bump(); ret token.COLON; }
        case ('?') { rdr.bump(); ret token.QUES; }
        case (';') { rdr.bump(); ret token.SEMI; }
        case (',') { rdr.bump(); ret token.COMMA; }
        case ('.') { rdr.bump(); ret token.DOT; }
        case ('(') { rdr.bump(); ret token.LPAREN; }
        case (')') { rdr.bump(); ret token.RPAREN; }
        case ('{') { rdr.bump(); ret token.LBRACE; }
        case ('}') { rdr.bump(); ret token.RBRACE; }
        case ('[') { rdr.bump(); ret token.LBRACKET; }
        case (']') { rdr.bump(); ret token.RBRACKET; }
        case ('@') { rdr.bump(); ret token.AT; }
        case ('#') { rdr.bump(); ret token.POUND; }
        case ('~') { rdr.bump(); ret token.TILDE; }


        // Multi-byte tokens.
        case ('=') {
            rdr.bump();
            if (rdr.curr() == '=') {
                rdr.bump();
                ret token.EQEQ;
            } else {
                ret token.EQ;
            }
        }

        case ('!') {
            rdr.bump();
            if (rdr.curr() == '=') {
                rdr.bump();
                ret token.NE;
            } else {
                ret token.NOT;
            }
        }

        case ('<') {
            rdr.bump();
            alt (rdr.curr()) {
                case ('=') {
                    rdr.bump();
                    ret token.LE;
                }
                case ('<') {
                    ret binop(rdr, token.LSL);
                }
                case ('-') {
                    rdr.bump();
                    ret token.LARROW;
                }
                case ('|') {
                    rdr.bump();
                    ret token.SEND;
                }
                case (_) {
                    ret token.LT;
                }
            }
        }

        case ('>') {
            rdr.bump();
            alt (rdr.curr()) {
                case ('=') {
                    rdr.bump();
                    ret token.GE;
                }

                case ('>') {
                    if (rdr.next() == '>') {
                        rdr.bump();
                        ret binop(rdr, token.ASR);
                    } else {
                        ret binop(rdr, token.LSR);
                    }
                }

                case (_) {
                    ret token.GT;
                }
            }
        }

        case ('\'') {
            rdr.bump();
            auto c2 = rdr.curr();
            if (c2 == '\\') {
                alt (rdr.next()) {
                    case ('n') { c2 = '\n'; }
                    case ('r') { c2 = '\r'; }
                    case ('t') { c2 = '\t'; }
                    case ('\\') { c2 = '\\'; }
                    case ('\'') { c2 = '\''; }

                    case ('x') { c2 = scan_numeric_escape(rdr); }
                    case ('u') { c2 = scan_numeric_escape(rdr); }
                    case ('U') { c2 = scan_numeric_escape(rdr); }

                    case (?c2) {
                        log_err #fmt("unknown character escape: %d",
                                     c2 as int);
                        fail;
                    }
                }
                rdr.bump();
            }

            if (rdr.next() != '\'') {
                log_err "unterminated character constant";
                fail;
            }
            rdr.bump(); // advance curr to closing '
            rdr.bump(); // advance curr past token
            ret token.LIT_CHAR(c2);
        }

        case ('"') {
            rdr.bump();
            while (rdr.curr() != '"') {
                alt (rdr.curr()) {
                    case ('\\') {
                        alt (rdr.next()) {
                            case ('n') {
                                rdr.bump();
                                _str.push_byte(accum_str, '\n' as u8);
                            }
                            case ('r') {
                                rdr.bump();
                                _str.push_byte(accum_str, '\r' as u8);
                            }
                            case ('t') {
                                rdr.bump();
                                _str.push_byte(accum_str, '\t' as u8);
                            }
                            case ('\\') {
                                rdr.bump();
                                _str.push_byte(accum_str, '\\' as u8);
                            }
                            case ('"') {
                                rdr.bump();
                                _str.push_byte(accum_str, '"' as u8);
                            }

                            case ('x') {
                                _str.push_char(accum_str,
                                               scan_numeric_escape(rdr));
                            }

                            case ('u') {
                                _str.push_char(accum_str,
                                               scan_numeric_escape(rdr));
                            }

                            case ('U') {
                                _str.push_char(accum_str,
                                               scan_numeric_escape(rdr));
                            }

                            case (?c2) {
                                log_err #fmt("unknown string escape: %d",
                                             c2 as int);
                                fail;
                            }
                        }
                    }
                    case (_) {
                        _str.push_char(accum_str, rdr.curr());
                    }
                }
                rdr.bump();
            }
            rdr.bump();
            ret token.LIT_STR(accum_str);
        }

        case ('-') {
            if (rdr.next() == '>') {
                rdr.bump();
                rdr.bump();
                ret token.RARROW;
            } else {
                ret binop(rdr, token.MINUS);
            }
        }

        case ('&') {
            if (rdr.next() == '&') {
                rdr.bump();
                rdr.bump();
                ret token.ANDAND;
            } else {
                ret binop(rdr, token.AND);
            }
        }

        case ('|') {
            if (rdr.next() == '|') {
                rdr.bump();
                rdr.bump();
                ret token.OROR;
            } else {
                ret binop(rdr, token.OR);
            }
        }

        case ('+') {
            ret binop(rdr, token.PLUS);
        }

        case ('*') {
            ret binop(rdr, token.STAR);
        }

        case ('/') {
            ret binop(rdr, token.SLASH);
        }

        case ('^') {
            ret binop(rdr, token.CARET);
        }

        case ('%') {
            ret binop(rdr, token.PERCENT);
        }

        case (?c) {
            log_err #fmt("unkown start of token: %d", c as int);
            fail;
        }
    }

    fail;
}

tag cmnt_ {
    cmnt_line(str);
    cmnt_block(vec[str]);
}

type cmnt = rec(cmnt_ val, uint pos, bool space_after);

fn consume_whitespace(reader rdr) -> uint {
    auto lines = 0u;
    while (is_whitespace(rdr.curr())) {
        if (rdr.curr() == '\n') {lines += 1u;}
        rdr.bump();
    }
    ret lines;
}

fn read_line_comment(reader rdr) -> cmnt {
    auto p = rdr.get_chpos();
    rdr.bump(); rdr.bump();
    while (rdr.curr() == ' ') {rdr.bump();}
    auto val = "";
    while (rdr.curr() != '\n' && !rdr.is_eof()) {
        _str.push_char(val, rdr.curr());
        rdr.bump();
    }
    ret rec(val=cmnt_line(val),
            pos=p,
            space_after=consume_whitespace(rdr) > 1u);
}

fn read_block_comment(reader rdr) -> cmnt {
    auto p = rdr.get_chpos();
    rdr.bump(); rdr.bump();
    while (rdr.curr() == ' ') {rdr.bump();}
    let vec[str] lines = vec();
    auto val = "";
    auto level = 1;
    while (true) {
        if (rdr.curr() == '\n') {
            _vec.push[str](lines, val);
            val = "";
            consume_whitespace(rdr);
        } else {
            if (rdr.curr() == '*' && rdr.next() == '/') {
                level -= 1;
                if (level == 0) {
                    rdr.bump(); rdr.bump();
                    _vec.push[str](lines, val);
                    break;
                }
            } else if (rdr.curr() == '/' && rdr.next() == '*') {
                level += 1;
            }
            _str.push_char(val, rdr.curr());
            rdr.bump();
        }
        if (rdr.is_eof()) {fail;}
    }
    ret rec(val=cmnt_block(lines),
            pos=p,
            space_after=consume_whitespace(rdr) > 1u);
}

fn gather_comments(str path) -> vec[cmnt] {
    auto srdr = io.file_reader(path);
    auto rdr = new_reader(srdr, path, codemap.new_filemap(path, 0u));
    let vec[cmnt] comments = vec();
    while (!rdr.is_eof()) {
        while (true) {
            consume_whitespace(rdr);
            if (rdr.curr() == '/' && rdr.next() == '/') {
                _vec.push[cmnt](comments, read_line_comment(rdr));
            } else if (rdr.curr() == '/' && rdr.next() == '*') {
                _vec.push[cmnt](comments, read_block_comment(rdr));
            } else { break; }
        }
        next_token(rdr);
    }
    ret comments;
}


//
// Local Variables:
// mode: rust
// fill-column: 78;
// indent-tabs-mode: nil
// c-basic-offset: 4
// buffer-file-coding-system: utf-8-unix
// compile-command: "make -k -C $RBUILD 2>&1 | sed -e 's/\\/x\\//x:\\//g'";
// End:
//
