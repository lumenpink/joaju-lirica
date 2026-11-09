//! Máscara binária para classes morfológicas.

use serde::{Deserialize, Serialize};

/// Máscara binária de 16 bits para representação compacta de classes morfológicas.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct PosMask(pub u16);

impl PosMask {
    /// Vocábulo sem classe identificada ou inexistente no léxico.
    pub const UNKNOWN: Self = Self(0);
    /// Substantivo comum ou próprio.
    pub const SUBSTANTIVO: Self = Self(1 << 0);
    /// Verbo flexionado ou forma nominal.
    pub const VERBO: Self = Self(1 << 1);
    /// Adjetivo qualificativo ou relacional.
    pub const ADJETIVO: Self = Self(1 << 2);
    /// Advérbio ou locução adverbial.
    pub const ADVERBIO: Self = Self(1 << 3);
    /// Pronome reto, oblíquo, possessivo, demonstrativo, etc.
    pub const PRONOME: Self = Self(1 << 4);
    /// Artigo definido ou indefinido.
    pub const ARTIGO: Self = Self(1 << 5);
    /// Preposição essencial ou acidental.
    pub const PREPOSICAO: Self = Self(1 << 6);
    /// Conjunção coordenativa ou subordinativa.
    pub const CONJUNCAO: Self = Self(1 << 7);
    /// Interjeição.
    pub const INTERJEICAO: Self = Self(1 << 8);
    /// Numeral cardinal, ordinal, etc.
    pub const NUMERAL: Self = Self(1 << 9);

    /// Retorna uma máscara vazia (todos os bits zerados).
    #[inline(always)]
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Constrói uma máscara a partir de bits brutos se forem válidos (<= 0x03FF).
    #[inline(always)]
    pub const fn from_bits(bits: u16) -> Option<Self> {
        if bits <= 0x03FF {
            Some(Self(bits))
        } else {
            None
        }
    }

    /// Retorna `true` se nenhum bit de classe morfológica estiver ativo.
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Retorna `true` se exatamente um bit de classe estiver ativo (sem ambiguidade).
    #[inline(always)]
    pub const fn is_resolved(&self) -> bool {
        self.0.count_ones() == 1
    }

    /// Retorna `true` se mais de um bit de classe estiver ativo (termo ambíguo).
    #[inline(always)]
    pub const fn is_ambiguous(&self) -> bool {
        self.0.count_ones() > 1
    }

    /// Retorna `true` se todos os bits de `other` estiverem contidos nesta máscara.
    #[inline(always)]
    pub const fn contains(&self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Retorna lista textual com o nome das classes ativas nesta máscara.
    pub fn nomes(&self) -> Vec<&'static str> {
        let mut v = Vec::new();
        if self.contains(Self::SUBSTANTIVO) {
            v.push("Substantivo");
        }
        if self.contains(Self::VERBO) {
            v.push("Verbo");
        }
        if self.contains(Self::ADJETIVO) {
            v.push("Adjetivo");
        }
        if self.contains(Self::ADVERBIO) {
            v.push("Advérbio");
        }
        if self.contains(Self::PRONOME) {
            v.push("Pronome");
        }
        if self.contains(Self::ARTIGO) {
            v.push("Artigo");
        }
        if self.contains(Self::PREPOSICAO) {
            v.push("Preposição");
        }
        if self.contains(Self::CONJUNCAO) {
            v.push("Conjunção");
        }
        if self.contains(Self::INTERJEICAO) {
            v.push("Interjeição");
        }
        if self.contains(Self::NUMERAL) {
            v.push("Numeral");
        }
        v
    }
}

impl std::ops::BitOr for PosMask {
    type Output = Self;
    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for PosMask {
    type Output = Self;
    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitOrAssign for PosMask {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}
