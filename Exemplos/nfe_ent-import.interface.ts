export interface FileProps {
  originalname: string
  buffer: Buffer
}

export interface AutXmlProps {
  CNPJ: string
  CPF?: string // Campo opcional
}

export interface IdeProps {
  cUF: string
  cNF: string
  natOp: string
  mod: string
  serie: string
  nNF: string
  dhEmi: string
  dhSaiEnt: string
  tpNF: string
  idDest: string
  cMunFG: string
  tpImp: string
  tpEmis: string
  cDV: string
  tpAmb: string
  finNFe: string
  indFinal: string
  indPres: string
  procEmi: string
  verProc: string
}

export interface EmitProps {
  CNPJ: string
  CPF?: string // Campo opcional
  xNome: string
  xFant: string
  enderEmit: {
    xLgr: string
    nro: string
    xCpl?: string // Campo opcional
    xBairro: string
    cMun: string
    xMun: string
    UF: string
    CEP: string
    cPais: string
    xPais: string
    fone: string
  }
  IE: string
  CRT: string
}

export interface DestProps {
  CNPJ: string
  CPF?: string // Campo opcional
  xNome: string
  enderDest: {
    xLgr: string
    nro: string
    xCpl?: string // Campo opcional
    xBairro: string
    cMun: string
    xMun: string
    UF: string
    CEP: string
    cPais: string
    xPais: string
  }
  indIEDest: string
  IE: string
  email: string
}

export interface ImpostoProps {
  vTotTrib: string
  ICMS: object
  IPI: object
  II: object
  PIS: object
  COFINS: object
}

export interface DetProps {
  nItem: string
  prod: {
    cProd: string
    cEAN: string
    xProd: string
    NCM: string
    CEST: string
    CFOP: string
    uCom: string
    qCom: string
    vUnCom: string
    vProd: string
    cEANTrib: string
    uTrib: string
    qTrib: string
    vUnTrib: string
    indTot: string
  }
  imposto: ImpostoProps
  infAdProd: string
}

export interface FatProps {
  nFat: string
  vOrig: string
  vDesc: string
  vLiq: string
}

export interface DupProps {
  nDup: string
  dVenc: string
  vDup: string
}

export interface CobrProps {
  fat: FatProps
  dup: DupProps
}

export interface DetPagProps {
  tPag: string
  vPag: string
}

export interface PagProps {
  detPag: DetPagProps
}

export interface TransportaProps {
  CNPJ: string
  xNome: string
  IE: string
  xEnder: string
  xMun: string
  UF: string
}

export interface VolProps {
  qVol?: string
  esp?: string
  pesoL?: string
  pesoB?: string
}

export interface TranspProps {
  modFrete: string
  transporta: TransportaProps
  retTransp?: {
    vServ: string
    vBCRet: string
    pICMSRet: string
    vICMSRet: string
    CFOP: string
    cMunFG: string
  }
  veicTransp?: {
    placa: string
    UF: string
    RNTC: string
  }
  reboque?: {
    placa: string
    UF: string
    RNTC: string
  }
  vagao?: {
    vagao: string
  }
  balsa?: {
    balsa: string
  }
  vol?: VolProps
}

export interface ObsContProps {
  xCampo: string
  xTexto: string
}

export interface InfAdicProps {
  infCpl: string
  obsCont: ObsContProps[]
}

export interface TotalProps {
  ICMSTot: {
    vBC: string
    vICMS: string
    vICMSDeson: string
    vFCPUFDest: string
    vICMSUFDest: string
    vICMSUFRemet: string
    vFCP: string
    vBCST: string
    vST: string
    vFCPST: string
    vFCPSTRet: string
    vProd: string
    vFrete: string
    vSeg: string
    vDesc: string
    vII: string
    vIPI: string
    vIPIDevol: string
    vPIS: string
    vCOFINS: string
    vOutro: string
    vNF: string
    vTotTrib: string
  }
}

export interface InfProtProps {
  tpAmb: string
  verAplic: string
  chNFe: string
  dhRecbto: string
  nProt: string
  digVal: string
  cStat: string
  xMotivo: string
}

export interface InfNFeProps {
  ide: IdeProps
  emit: EmitProps
  dest: DestProps
  autXML: AutXmlProps
  det: DetProps[]
  transp: TranspProps
  total: TotalProps
  infAdic: InfAdicProps
  // pag: PagProps
}

export interface NFeProps {
  infNFe: InfNFeProps
}

export interface ProtNFeProps {
  infProt: InfProtProps
}

export interface NFeProcProps {
  NFe: NFeProps
  protNFe: ProtNFeProps
}

export interface DataProps {
  nfeProc: NFeProcProps
}

export interface NFeDataProps {
  ide: IdeProps
  emit: EmitProps
  dest: DestProps
  autXML: AutXmlProps
  det: DetProps[]
  transp: TranspProps
  total: TotalProps
  // pag: PagProps
  infAdic: InfAdicProps
  infProt: InfProtProps
}
