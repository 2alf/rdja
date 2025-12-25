користи proc_macro::{Group, Ident, TokenStream, TokenTree};

фк замени_идент(идент: Ident) -> Опција<TokenTree> {
    нека идент_низ = идент.to_string();

    нека нови_низ = упореди идент_низ.as_str() {
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


        _ => &идент_низ,
    };

    нека нови_идент = Ident::new(нови_низ, идент.span());
    Неке(TokenTree::Ident(нови_идент))
}

фк замени_стабло(ток: TokenTree, излаз: &променљиво Vec<TokenTree>) {
    упореди ток {
        TokenTree::Group(група) => {
            нека променљиво елем_групе = Vec::new();
            замени_ток(група.stream(), &променљиво елем_групе);
            нека променљиво нови_ток = TokenStream::new();
            нови_ток.extend(елем_групе);
            излаз.push(TokenTree::Group(Group::new(група.delimiter(), нови_ток)));
        }
        TokenTree::Ident(идент) => {
            ако нека Неке(идент) = замени_идент(идент) {
                излаз.push(идент);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            излаз.push(ток);
        }
    }
}

фк замени_ток(тс: TokenStream, излаз: &променљиво Vec<TokenTree>) {
    за ток у тс {
        замени_стабло(ток, излаз)
    }
}

#[proc_macro]
јавно фк rust(ставка: TokenStream) -> TokenStream {
    нека променљиво враћено = Vec::new();
    замени_ток(ставка, &променљиво враћено);
    нека променљиво излаз = TokenStream::new();
    излаз.extend(враћено);
    излаз
}
