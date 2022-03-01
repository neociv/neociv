pub type Alignment = f32;

#[derive(Clone, Debug)]
pub struct Alignments {
    /// Globalism (0) to Nationalism (1)
    pub glob_nat: Alignment,
    /// Capitalism (0) to Socialism (1)
    pub cap_soc: Alignment,
    /// Anarchy (0) to Authoritarianism (1)
    pub anc_auth: Alignment,
    /// Pacifism (0) to Aggressive (1)
    pub pac_agro: Alignment,
}

impl Default for Alignments {
    fn default() -> Self {
        return Alignments {
            glob_nat: 1.0,
            cap_soc: 0.0,
            anc_auth: 0.0,
            pac_agro: 1.0,
        };
    }
}

#[test]
fn test_defaults() {
    let alignments = Alignments::default();
    assert_eq!(alignments.glob_nat, 1.0);
    assert_eq!(alignments.cap_soc, 0.0);
    assert_eq!(alignments.anc_auth, 0.0);
    assert_eq!(alignments.pac_agro, 1.0);
}

