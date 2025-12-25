use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        // ћирилица
        "Упс" | "Грешка"  => "Err",
        "Ок" | "Уреду" | "Како_кажеш" => "Ok",
        "Низ" => "String",
        "ХашМапа" => "HashMap",
        "Подразумевано" => "Default",
        "ТипГрешке" => "Error",
        "Опција" => "Option",
        "Неке" => "Some",
        "Ниједно" => "None",
        "Резултат" => "Result",
        "Сам" => "Self",
        "пишиЛинију" => "println",
        "прекини" => "break",
        "асинхроно" => "async",
        "чекај" => "await",
        "петља" => "loop",
        "помери" => "move",
        "сандук" => "crate",
        "недостижан_код" => "unreachable_code",
        "као" => "as",
        "конст" => "const",
        "особина" => "trait",
        "несигурно" => "unsafe",
        "у" => "in",
        "из" => "from",
        "динамички" => "dyn",
        "распакуј" => "unwrap",
        "подразумевано" => "default",
        "као_реф" => "as_ref",
        "ио" => "io",
        "спољашњи" => "extern",
        "нетачно" => "false",
        "фк" => "fn",
        "супер" => "super",
        "убаци" => "insert",
        "узми" => "get",
        "дозволи" => "allow",
        "срање" | "зајеб" | "паничи" => "panic",
        "модул" => "mod",
        "променљиво" => "mut",
        "ново" => "new",
        "где" => "where",
        "за" => "for",
        "узми_или_убаци_са" => "get_or_insert_with",
        "главно" => "main",
        "јавно" => "pub",
        "врати" => "return",
        "импл" => "impl",
        "реф" => "ref",
        "упореди" => "match",
        "ако" => "if",
        "иначе" => "else",
        "сам" => "self",
        "нека" => "let",
        "статичко" => "static",
        "структура" => "struct",
        "очекуј" => "expect",
        "док" => "while",
        "користи" => "use",
        "у_ово" => "into",
        "тачно" => "true",
        "булово" | "логичко" => "bool",
        "наброи" => "enum",

        // latinica
        "Ups" | "Greška"  => "Err",
        "Ok" | "Uredu" | "Kako_kažeš" => "Ok",
        "Niz" => "String",
        "HašMapa" => "HashMap",
        "Podrazumevano" => "Default",
        "TipGreške" => "Error",
        "Opcija" => "Option",
        "Neke" => "Some",
        "Nijedno" => "None",
        "Rezultat" => "Result",
        "Sam" => "Self",
        "pišiLiniju" => "println",
        "prekini" => "break",
        "asinhrono" => "async",
        "čekaj" => "await",
        "petlja" => "loop",
        "pomeri" => "move",
        "sanduk" => "crate",
        "nedostižan_kod" => "unreachable_code",
        "kao" => "as",
        "konst" => "const",
        "osobina" => "trait",
        "nesigurno" => "unsafe",
        "u" => "in",
        "iz" => "from",
        "dinamički" => "dyn",
        "raspakuj" => "unwrap",
        "podrazumevano" => "default",
        "kao_ref" => "as_ref",
        "io" => "io",
        "spoljašnji" => "extern",
        "netačno" => "false",
        "fk" => "fn",
        "super" => "super",
        "ubaci" => "insert",
        "uzmi" => "get",
        "dozvoli" => "allow",
        "sranje" | "zajeb" | "paniči" => "panic",
        "modul" => "mod",
        "promenljivo" => "mut",
        "novo" => "new",
        "gde" => "where",
        "za" => "for",
        "uzmi_ili_ubaci_sa" => "get_or_insert_with",
        "glavno" => "main",
        "javno" => "pub",
        "vrati" => "return",
        "impl" => "impl",
        "uporedi" => "match",
        "ako" => "if",
        "inače" => "else",
        "sam" => "self",
        "neka" => "let",
        "statičko" => "static",
        "struktura" => "struct",
        "očekuj" => "expect",
        "dok" => "while",
        "koristi" => "use",
        "u_ovo" => "into",
        "tačno" => "true",
        "bulovo" | "logičko" => "bool",
        "nabroi" => "enum",



        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn rust(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
