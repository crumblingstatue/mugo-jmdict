use mugo::RootKind;

pub trait RootKindExt {
    fn to_jmdict_part_of_speech(&self) -> jmdict::PartOfSpeech;
}

impl RootKindExt for RootKind {
    fn to_jmdict_part_of_speech(&self) -> jmdict::PartOfSpeech {
        use jmdict::PartOfSpeech as Part;
        match self {
            RootKind::Ichidan => Part::IchidanVerb,
            RootKind::GodanBu => Part::GodanBuVerb,
            RootKind::GodanMu => Part::GodanMuVerb,
            RootKind::GodanNu => Part::GodanNuVerb,
            RootKind::GodanRu => Part::GodanRuVerb,
            RootKind::GodanSu => Part::GodanSuVerb,
            RootKind::GodanTsu => Part::GodanTsuVerb,
            RootKind::GodanU => Part::GodanUVerb,
            RootKind::GodanGu => Part::GodanGuVerb,
            RootKind::GodanKu => Part::GodanKuVerb,
            RootKind::IAdjective => Part::Adjective,
            RootKind::Iku => Part::GodanIkuVerb,
            RootKind::Kuru => Part::KuruVerb,
            RootKind::NaAdjective => Part::AdjectivalNoun,
            RootKind::Suru => Part::SuruVerb,
            RootKind::SpecialSuru => Part::SpecialSuruVerb,
        }
    }
}

pub fn root_kind_matches(kind: &mugo::RootKind, mut senses: jmdict::Senses) -> bool {
    senses.any(|sense| {
        sense
            .parts_of_speech()
            .any(|part| part == kind.to_jmdict_part_of_speech())
    })
}

pub enum Root<'a> {
    Bare(&'a str),
    Conj(&'a mugo::Root),
}

impl<'a> Root<'a> {
    pub fn text_matches(&self, text: &str) -> bool {
        match *self {
            Root::Bare(s) => text == s,
            Root::Conj(root) => {
                let mut matches = root.dict_string() == text;
                if matches!(root.kind, mugo::RootKind::Suru) {
                    matches |= root.text == text;
                }
                matches
            }
        }
    }

    pub fn matches(&self, e: &jmdict::Entry) -> bool {
        match self {
            Root::Bare(_) => self.reading_matches(e),
            Root::Conj(root) => {
                root_kind_matches(&root.kind, e.senses()) && self.reading_matches(e)
            }
        }
    }

    pub fn reading_matches(&self, e: &jmdict::Entry) -> bool {
        e.reading_elements().any(|e| self.text_matches(e.text))
            || e.kanji_elements().any(|e| self.text_matches(e.text))
    }
}
