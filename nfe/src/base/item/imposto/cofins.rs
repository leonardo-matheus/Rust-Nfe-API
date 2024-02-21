// Grupos COFINS
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

// COFINS
#[derive(Debug, PartialEq, Clone)]

pub enum GrupoCofins {
    // Outras Operações
    CofinsOutr(GrupoCofinsOutr),
    // Não Tributado
    CofinsNt(GrupoCofinsNt),
    // Operação Tributável pela Alíquota Básica
    CofinsAliq(GrupoCofinsAliq),
}

impl<'de> Deserialize<'de> for GrupoCofins {
    fn deserialize<D>(deserializer: D) -> Result<GrupoCofins, D::Error>
    where
        D: Deserializer<'de>,
    {
        let grc: GrupoCofinsContainer = GrupoCofinsContainer::deserialize(deserializer)?;

        if let Some(gr: GrupoCofinsOutr) = grc.cofins_outr {
            return Ok(GrupoCofins::CofinsOutr(gr));
        }

        if let Some(gr: GrupoCofinsNt) = grc.cofins_nt {
            return Ok(GrupoCofins::CofinsNt(gr));
        }

        if let Some(gr: GrupoCofinsAliq) = grc.cofins_aliq {
            return Ok(GrupoCofins::CofinsAliq(gr));
        }

        Err(D::Error::custom(msg:"Grupo de COFINS não suportado".to_string()))
    }
}

impl Serialize for GrupoCofins {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let grc = match self {
            GrupoCofins::CofinsOutr(g) => GrupoCofinsContainer {
                cofins_outr: Some(g.clone()),
                cofins_nt: None,
                cofins_aliq: None,
            },
            GrupoCofins::CofinsNt(g) => GrupoCofinsContainer {
                cofins_outr: None,
                cofins_nt: Some(g.clone()),
                cofins_aliq: None,
            },
            GrupoCofins::CofinsAliq(g) => GrupoCofinsContainer {
                cofins_outr: None,
                cofins_nt: None,
                cofins_aliq: Some(g.clone()),
            },
        };

        grc.serialize(serializer)
    }
}