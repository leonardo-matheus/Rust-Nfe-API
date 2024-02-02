//! Modelo 55 - NFe

use crate::base::dest::Destinatario as DestinatarioBase;
use crate::base::emit::*;
use crate::base::endereco::*;
use crate::base::ide::*;
use crate::base::item::*;
use crate::base::totais::*;
use crate::base::transporte::*;
use crate::base::Nfe as NfeBase;
use crate::base::VersaoLayout;
use std::convert::{TryFrom, TryInto};
use std::fs::File;
use std::str::FromStr;

mod dest;
mod error;
pub use dest::*;
pub use error::*;

// Nota Fiscal Eletrônica
pub struct Nfe {
    pub versao: VersaoLayout,
    pub chave_acesso: String,
    pub ide: Identificacao,
    pub emit: Emitente,
    pub dest: Destinatario,
    pub itens: Vec<Item>,
    pub totais: Totalizacao,
    pub transporte: Transporte,
    // Informações adicionais
    pub informacao_complementar: Option<String>,
}

impl TryFrom<NfeBase> for Nfe {
    type Error = error::Error;

    fn try_from(base: NfeBase) -> Result<Self, Self::Error> {
        if base.ide.modelo != ModeloDocumentoFiscal::Nfe {
            return Err(error::Error::ModeloInvalido(base.ide.modelo));
        }

        let dest = doc
            .dest
            .ok_or_else(|| Error::DestinatarioInvalido("Não informado no documento".to_string()))?
            .try_into()?;

        Ok(Self {
            versao: doc.versao,
            chave_acesso: doc.chave_acesso,
            ide: doc.ide,
            emit: doc.emit,
            dest,
            itens: doc.itens,
            totais: doc.totais,
            transporte: doc.transporte,
            informacao_complementar: doc.informacao_complementar,
        })
    }
}

impl From<&Nfe> for NfeBase {
    fn from(doc: &Nfe) -> Self {
        let dest: DestinatarioBase = (&doc.dest).into();

        Self {
            versao: doc.versao.clone(),
            chave_acesso: doc.chave_acesso.clone(),
            ide: doc.ide.clone(),
            emit: doc.emit.clone(),
            dest: Some(dest),
            itens: doc.itens.clone(),
            totais: doc.totais.clone(),
            transporte: doc.transporte.clone(),
            informacao_complementar: doc.informacao_complementar.clone(),
        }
    }
}
fn from(nfe: &Nfe) -> Self {
    Self {
        versao: nfe.versao,
        chave_acesso: nfe.chave_acesso.clone(),
        ide: nfe.ide.clone(),
        emit: nfe.emit.clone(),
        dest: Some(dest),
        itens: nfe.itens.clone(),
        totais: nfe.totais.clone(),
        transporte: nfe.transporte.clone(),
        informacao_complementar: nfe.informacao_complementar.clone(),
    }
}

// Implementação de conversão de &Nfe para NfeBase
impl FromStr for Nfe {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<NfeBase>()?.try_into()
    }
}

impl TryFrom<File> for Nfe {
    type Error = Error;

    fn try_from(f: File) -> Result<Self, Self::Error> {
        NfeBase::try_from(f)?.try_into()
    }
}

impl ToString for Nfe {
    fn to_string(&self) -> String {
        let base: NfeBase = self.into();

        base.to_string()
    }
}
