koristi proc_macro::{Group, Ident, TokenStream, TokenTree};

fk zameni_ident(ident: Ident) -> Opcija<TokenTree> {
    neka ident_niz = ident.to_string();

    neka novi_niz = uporedi ident_niz.as_str() {
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


        _ => &ident_niz,
    };

    neka novi_ident = Ident::new(novi_niz, ident.span());
    Neke(TokenTree::Ident(novi_ident))
}

fk zameni_stablo(tok: TokenTree, izlaz: &promenljivo Vec<TokenTree>) {
    uporedi tok {
        TokenTree::Group(grupa) => {
            neka promenljivo elem_grupe = Vec::new();
            zameni_tok(grupa.stream(), &promenljivo elem_grupe);
            neka promenljivo novi_tok = TokenStream::new();
            novi_tok.extend(elem_grupe);
            izlaz.push(TokenTree::Group(Group::new(grupa.delimiter(), novi_tok)));
        }
        TokenTree::Ident(ident) => {
            ako neka Neke(ident) = zameni_ident(ident) {
                izlaz.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            izlaz.push(tok);
        }
    }
}

fk zameni_tok(ts: TokenStream, izlaz: &promenljivo Vec<TokenTree>) {
    za tok u ts {
        zameni_stablo(tok, izlaz)
    }
}

#[proc_macro]
javno fk rust(stavka: TokenStream) -> TokenStream {
    neka promenljivo vraćeno = Vec::new();
    zameni_tok(stavka, &promenljivo vraćeno);
    neka promenljivo izlaz = TokenStream::new();
    izlaz.extend(vraćeno);
    izlaz
}
