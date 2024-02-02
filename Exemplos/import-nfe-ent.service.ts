import * as xml2js from 'xml2js'
import {
  BadRequestException,
  ConflictException,
  Injectable,
} from '@nestjs/common'
import {
  FileProps,
  NFeDataProps,
  DataProps,
} from '../interface/nfe_ent-import.interface'
import { PrismaService } from '@/prisma/prisma.service'
import { UserPayload } from '@/modulos/auth/jwt.strategy'
import { handleErrors } from '@/share/error/error-handler'
import { PrismaErrorHandler } from '@/share/error/error-handler-prisma'
import { getAuthorizedCompanys } from '@/share/company/empresa-autorized-utils'
import { PrismaPromise, NFeEntItens } from '@prisma/client'
import { hasNestedProperty } from '@/share/check/check-nested-property'
import { doFmtRazaoCnpj, doFormatarCNPJorCPF } from '@/Share/format/fmt'
import { TipoCadastro } from '@/enums/tipo-cadastro.enum'

// ########################################################################## //
@Injectable()
export class ImportNFeEntService {
  constructor(private prisma: PrismaService) {}

  async saveXmlToDatabase(
    file: FileProps,
    userCurrentAuth: UserPayload,
  ): Promise<void> {
    let data
    try {
      // step 1: get company id
      const cEmpIds = await getAuthorizedCompanys(
        this.prisma,
        userCurrentAuth.sub,
      )

      if (!file) {
        throw new BadRequestException('Arquivo inválido.')
      }
      const xmlFile = file.buffer.toString('utf8')
      if (!xmlFile) {
        throw new BadRequestException('XmlFile inválido.')
      }

      try {
        data = await xml2js.parseStringPromise(xmlFile, {
          mergeAttrs: true,
          explicitArray: false,
        })
      } catch (error) {
        throw new BadRequestException(
          'Erro ao analisar o arquivo XML: ' + error,
        )
      }

      if (!data.nfeProc || !data.nfeProc.NFe) {
        throw new BadRequestException('NFe não encontrada no arquivo XML')
      }

      const nFeEnt = this.mapToNFeData(data)
      // step 3: check if nfe is already registered
      await this.doCheckNFeEntCadastrada(
        nFeEnt.ide.nNF,
        nFeEnt.ide.serie,
        nFeEnt.ide.tpAmb,
        cEmpIds[0],
        nFeEnt.infProt.chNFe,
      )
      // step 4: create nfe_ent
      await this.doImportNFeEnt(nFeEnt, cEmpIds[0], xmlFile, userCurrentAuth)
      // step 5: return
    } catch (error) {
      PrismaErrorHandler.handlePrismaError(error)
      handleErrors(error)
    }
  }

  // ###################################################################### //
  private async doImportNFeEnt(
    nFeEnt: NFeDataProps,
    cEmpId: string,
    xmlFile: string,
    userCurrentAuth: UserPayload,
  ) {
    let lCadForn
    try {
      // ============================================================== //
      //                  step 1.1: create nfe_ent                      //
      // ============================================================== //
      const cNFeEnt = this.prisma.nFeEnt.create({
        data: {
          nnf: parseInt(nFeEnt.ide.nNF),
          serie: parseInt(nFeEnt.ide.serie),
          namb: parseInt(nFeEnt.ide.tpAmb),
          empId: cEmpId,
          chaveacesso: nFeEnt.infProt?.chNFe,
          ideDemi: nFeEnt.ide.dhEmi,
          emitCnpjcpf: doFormatarCNPJorCPF(nFeEnt.emit?.CNPJ),
          emitXnome: nFeEnt.emit?.xNome,
          destCnpjCpf: doFormatarCNPJorCPF(nFeEnt.dest?.CNPJ),
          destXnome: nFeEnt.dest?.xNome,
          totalIcmstotVprod: nFeEnt.total?.ICMSTot?.vProd,
          totalIcmstotVnf: nFeEnt.total?.ICMSTot?.vNF,
          nfeNprot: nFeEnt.infProt?.nProt,
          nfeCstat: parseInt(nFeEnt.infProt?.cStat),
          nfeDatarec: nFeEnt.infProt?.dhRecbto,
          nfeXmotivo: nFeEnt.infProt?.xMotivo,
          nfeXml: xmlFile,
          logUserCad: userCurrentAuth.sub,
        },
      })
      // ============================================================== //
      //                  step 1.2: create nfe_ent_ide                  //
      // ============================================================== //
      const cNFeEntIde = this.prisma.nFeEntIde.create({
        data: {
          nnf: parseInt(nFeEnt.ide.nNF),
          serie: parseInt(nFeEnt.ide.serie),
          namb: parseInt(nFeEnt.ide.tpAmb),
          empId: cEmpId,
          chaveacesso: nFeEnt.infProt?.chNFe,
          // Fields -> NFeEntIde
          ideCuf: parseInt(nFeEnt.ide.cUF),
          ideCnf: parseInt(nFeEnt.ide.cNF),
          ideNatopDesc: nFeEnt.ide.natOp,
          ideMod: parseInt(nFeEnt.ide.mod),
          ideDemi: nFeEnt.ide.dhEmi,
          ideDhsaient: nFeEnt.ide.dhSaiEnt,
          ideTpnfCod: nFeEnt.ide.tpNF,
          ideCmunfg: parseInt(nFeEnt.ide.cMunFG),
          ideIddestCod: nFeEnt.ide.idDest,
          //  idecMunFG
          ideTpimpCod: nFeEnt.ide.tpImp,
          ideTpemisCod: nFeEnt.ide.tpEmis,
          ideCdv: parseInt(nFeEnt.ide.cDV),
          ideFinnfeCod: nFeEnt.ide.finNFe,
          ideIndfinalCod: nFeEnt.ide.indFinal,
          ideIndpresCod: nFeEnt.ide.indPres,
          ideProcemi: nFeEnt.ide.procEmi,
          ideVerproc: nFeEnt.ide.verProc,
          logUserCad: userCurrentAuth.sub,
        },
      })

      // ============================================================== //
      //                 step 1.3: create nfe_ent_emit                  //
      // ============================================================== //
      const cNFeEntEmit = this.prisma.nFeEntEmit.create({
        data: {
          // pk_nfeent_emit
          nnf: parseInt(nFeEnt.ide.nNF),
          serie: parseInt(nFeEnt.ide.serie),
          namb: parseInt(nFeEnt.ide.tpAmb),
          empId: cEmpId,
          chaveacesso: nFeEnt.infProt.chNFe,
          // Fields -> NFeEntEmit
          emitCnpjcpf: doFormatarCNPJorCPF(nFeEnt.emit?.CNPJ),
          emitXnome: nFeEnt.emit?.xNome,
          emitXfant: nFeEnt.emit?.xFant,
          emitEnderemitXlgr: nFeEnt.emit?.enderEmit?.xLgr,
          emitEnderemitNro: nFeEnt.emit?.enderEmit?.nro,
          emitEnderemitXbairro: nFeEnt.emit?.enderEmit?.xBairro,
          emitEnderemitCmun: nFeEnt.emit?.enderEmit
            ? parseInt(nFeEnt.emit.enderEmit.cMun)
            : null,
          emitEnderemitXmun: nFeEnt.emit?.enderEmit?.xMun,
          emitEnderemitUf: nFeEnt.emit?.enderEmit?.UF,
          emitEnderemitCep: nFeEnt.emit?.enderEmit?.CEP,
          emitEnderemitCpais: nFeEnt.emit?.enderEmit
            ? parseInt(nFeEnt.emit.enderEmit.cPais)
            : null,
          emitEnderemitXpais: nFeEnt.emit?.enderEmit?.xPais,
          emitEnderemitFone: nFeEnt.emit?.enderEmit?.fone,
          emitIe: nFeEnt.emit?.IE,
          emitCrtCod: nFeEnt.emit?.CRT,
          logUserCad: userCurrentAuth.sub,
        },
      })

      const cExisteForn = await this.prisma.cliforn.findUnique({
        where: {
          empId_cnpjCpf: {
            empId: cEmpId,
            cnpjCpf: doFormatarCNPJorCPF(nFeEnt.emit?.CNPJ),
          },
        },
      })
      console.log('cExisteForn', cExisteForn)
      if (!cExisteForn) {
        lCadForn = this.prisma.cliforn.create({
          data: {
            empId: cEmpId,
            tipopessoa:
              nFeEnt.emit?.CNPJ.replace(/\D/g, '').length > 11 ? 'J' : 'F',
            razaoCnpj: doFmtRazaoCnpj(nFeEnt.emit?.xNome, nFeEnt.emit?.CNPJ),
            tipocadId: TipoCadastro.tcFornecedor.tipocad_id,
            cnpjCpf: doFormatarCNPJorCPF(nFeEnt.emit?.CNPJ),
            razaoNome: nFeEnt.emit?.xNome.toUpperCase(),
            isFornecedor: true,
            logUserCad: userCurrentAuth.sub,
            permitidoEditDel: true,
            permitido: true,

            // razaoCnpj: doFormatarCNPJorCPF(nFeEnt.emit?.CNPJ),
            // fornXnome: nFeEnt.emit?.xNome,
            // fornXfant: nFeEnt.emit?.xFant,
            // fornEnderXlgr: nFeEnt.emit?.enderEmit?.xLgr,
            // fornEnderNro: nFeEnt.emit?.enderEmit?.nro,
            // fornEnderXbairro: nFeEnt.emit?.enderEmit?.xBairro,
            // fornEnderCmun: nFeEnt.emit?.enderEmit
            //   ? parseInt(nFeEnt.emit.enderEmit.cMun)
            //   : null,
            // fornEnderXmun: nFeEnt.emit?.enderEmit?.xMun,
            // fornEnderUf: nFeEnt.emit?.enderEmit?.UF,
            // fornEnderCep: nFeEnt.emit?.enderEmit?.CEP,
            // fornEnderCpais: nFeEnt.emit?.enderEmit
            //   ? parseInt(nFeEnt.emit.enderEmit.cPais)
            //   : null,
            // fornEnderXpais: nFeEnt.emit?.enderEmit?.xPais,
            // fornEnderFone: nFeEnt.emit?.enderEmit?.fone,
            // fornIe: nFeEnt.emit?.IE,
            // fornCrtCod: nFeEnt.emit?.CRT,
            // logUserCad: userCurrentAuth.sub,
          },
        })
      }

      // ============================================================== //
      //                 step 1.4: create nfe_ent_dest                  //
      // ============================================================== //
      const cNFeEntDest = this.prisma.nFeEntDest.create({
        data: {
          // pk_nfeent_dest
          nnf: parseInt(nFeEnt.ide.nNF),
          serie: parseInt(nFeEnt.ide.serie),
          namb: parseInt(nFeEnt.ide.tpAmb),
          empId: cEmpId,
          chaveacesso: nFeEnt.infProt.chNFe,
          // Fields -> NFeEntDest
          destCnpjCpf: doFormatarCNPJorCPF(nFeEnt.dest?.CNPJ),
          destXnome: nFeEnt.dest?.xNome,
          destEnderdestXlgr: nFeEnt.dest?.enderDest?.xLgr,
          destEnderdestNro: nFeEnt.dest?.enderDest?.nro,
          destEnderdestXbairro: nFeEnt.dest?.enderDest?.xBairro,
          destEnderdestCmun: nFeEnt.dest?.enderDest
            ? parseInt(nFeEnt.dest.enderDest.cMun)
            : null,
          destEnderdestXmun: nFeEnt.dest?.enderDest?.xMun,
          destEnderdestUf: nFeEnt.dest?.enderDest?.UF,
          destEnderdestCep: nFeEnt.dest?.enderDest?.CEP,
          destEnderdestCpais: nFeEnt.dest?.enderDest
            ? parseInt(nFeEnt.dest.enderDest.cPais)
            : null,
          destEnderdestXpais: nFeEnt.dest?.enderDest?.xPais,
          destIndiedestCod: nFeEnt.dest?.indIEDest,
          destIe: nFeEnt.dest?.IE,
          destEmail: nFeEnt.dest?.email,
          logUserCad: userCurrentAuth.sub,
        },
      })
      // ============================================================== //
      //                 step 1.5: create nfe_ent_autxml                //
      // ============================================================== //
      const cNFeEntAutXml = this.prisma.nFeEntAutXml.create({
        data: {
          // pk_nfeent_emit
          nnf: parseInt(nFeEnt.ide.nNF),
          serie: parseInt(nFeEnt.ide.serie),
          namb: parseInt(nFeEnt.ide.tpAmb),
          empId: cEmpId,
          chaveacesso: nFeEnt.infProt.chNFe,
          // Fields -> NFeEntEmit
          autxmlCnpjcpf: nFeEnt.autXML?.CNPJ,
          logUserCad: userCurrentAuth.sub,
        },
      })
      // ============================================================== //
      //                 step 1.6: create nfe_ent_itens                 //
      // ============================================================== //
      const cNFeEntItens: PrismaPromise<NFeEntItens>[] = []

      for (let i = 0; i < nFeEnt.det.length; i++) {
        const createOperation = this.prisma.nFeEntItens.create({
          data: {
            // pk_nfeent_emit
            nnf: parseInt(nFeEnt.ide.nNF),
            serie: parseInt(nFeEnt.ide.serie),
            namb: parseInt(nFeEnt.ide.tpAmb),
            empId: cEmpId,
            chaveacesso: nFeEnt.infProt.chNFe,
            item: parseInt(nFeEnt.det[i]?.nItem),
            // Fields -> NFeEntItens
            prodCprod: nFeEnt.det[i]?.prod.cProd,
            prodXprod: nFeEnt.det[i]?.prod.xProd,
            prodNcmCod: nFeEnt.det[i]?.prod.NCM,
            prodCfopCod: parseInt(nFeEnt.det[i]?.prod.CFOP),
            prodUcom: nFeEnt.det[i]?.prod.uCom,
            prodQcom: parseFloat(nFeEnt.det[i]?.prod.qCom),
            prodVuncom: parseFloat(nFeEnt.det[i]?.prod.vUnCom),
            prodVprod: parseFloat(nFeEnt.det[i]?.prod.vProd),
            prodUtrib: nFeEnt.det[i]?.prod.uTrib,
            prodQtrib: parseFloat(nFeEnt.det[i]?.prod.qTrib),
            prodVuntrib: parseFloat(nFeEnt.det[i]?.prod.vUnTrib),
            // prodVfrete: parseFloat(nFeEnt.det[i]?.vFrete),
            // prodVseg: parseFloat(nFeEnt.det[i]?.vSeg),
            // prodVdesc: parseFloat(nFeEnt.det[i]?.vDesc),
            // prodVoutro: parseFloat(nFeEnt.det[i]?.vOutro),
            prodIndtot: parseInt(nFeEnt.det[i]?.prod.indTot),
            logUserCad: userCurrentAuth.sub,
          },
        })
        cNFeEntItens.push(createOperation)
      }
      // ============================================================== //
      //                 step 1.7: create nfe_ent_transp                //
      // ============================================================== //
      const cNFeEntTransp = this.prisma.nFeEntTransp.create({
        data: {
          // pk_nfeent_total
          nnf: parseInt(nFeEnt.ide.nNF),
          serie: parseInt(nFeEnt.ide.serie),
          namb: parseInt(nFeEnt.ide.tpAmb),
          empId: cEmpId,
          chaveacesso: nFeEnt.infProt.chNFe,
          // Fields -> NFeEntTransp
          modfreteCod: nFeEnt.transp?.modFrete,
          transpTransportaCnpjcpf: nFeEnt.transp?.transporta?.CNPJ,
          transpTransportaXnome: nFeEnt.transp?.transporta?.xNome,
          transpTransportaIe: nFeEnt.transp?.transporta?.IE,
          transpTransportaXender: nFeEnt.transp?.transporta?.xEnder,
          transpTransportaXmun: nFeEnt.transp?.transporta?.xMun,
          transpTransportaUf: nFeEnt.transp?.transporta?.UF,
          // transpRettranspVserv: nFeEnt.transp?.retTransp?.vServ,
          // transpRettranspVbcret: nFeEnt.transp?.retTransp?.vBCRet,
          // transpRettranspPicmsret: nFeEnt.transp?.retTransp?.pICMSRet,
          // transpRettranspVicmsret: nFeEnt.transp?.retTransp?.vICMSRet,
          // transpRettranspCfop: parseInt(nFeEnt.transp?.retTransp?.CFOP),
          // transpRettranspCmunfg: parseInt(nFeEnt.transp?.retTransp?.cMunFG),
          // transpVeictranspPlaca: nFeEnt.transp?.veicTransp?.placa,
          // transpVeictranspUf: nFeEnt.transp?.veicTransp?.UF,
          // transpVeictranspRntc: nFeEnt.transp?.veicTransp?.RNTC,
          // transpVagao: nFeEnt.transp?.vagao.vagao,
          // transpBalsa: nFeEnt.transp?.balsa.balsa,
          logUserCad: userCurrentAuth.sub,
        },
      })
      // ============================================================== //
      //                 step 1.8: create nfe_ent_total                  //
      // ============================================================== //
      const cNFeEntTotal = this.prisma.nFeEntTotal.create({
        data: {
          // pk_nfeent_total
          nnf: parseInt(nFeEnt.ide.nNF),
          serie: parseInt(nFeEnt.ide.serie),
          namb: parseInt(nFeEnt.ide.tpAmb),
          empId: cEmpId,
          chaveacesso: nFeEnt.infProt.chNFe,
          // Fields -> NFeEntTotal
          totalIcmstotVbc: nFeEnt.total?.ICMSTot?.vBC,
          totalIcmstotVicms: nFeEnt.total?.ICMSTot?.vICMS,
          totalIcmstotVbcst: nFeEnt.total?.ICMSTot?.vBCST,
          totalIcmstotVst: nFeEnt.total?.ICMSTot?.vST,
          totalIcmstotVprod: nFeEnt.total?.ICMSTot?.vProd,
          totalIcmstotVfrete: nFeEnt.total?.ICMSTot?.vFrete,
          totalIcmstotVseg: nFeEnt.total?.ICMSTot?.vSeg,
          totalIcmstotVdesc: nFeEnt.total?.ICMSTot?.vDesc,
          totalIcmstotVii: nFeEnt.total?.ICMSTot?.vII,
          totalIcmstotVipi: nFeEnt.total?.ICMSTot?.vIPI,
          totalIcmstotVpis: nFeEnt.total?.ICMSTot?.vPIS,
          totalIcmstotVcofins: nFeEnt.total?.ICMSTot?.vCOFINS,
          totalIcmstotVoutro: nFeEnt.total?.ICMSTot?.vOutro,
          totalIcmstotVnf: nFeEnt.total?.ICMSTot?.vNF,
          logUserCad: userCurrentAuth.sub,
        },
      })
      // ============================================================== //
      //                 step 1.9: create nfe_ent_infadic               //
      // ============================================================== //
      const cNFeEntInfadic = this.prisma.nFeEntInfadic.create({
        data: {
          // pk_nfeent_infoadic
          nnf: parseInt(nFeEnt.ide.nNF),
          serie: parseInt(nFeEnt.ide.serie),
          namb: parseInt(nFeEnt.ide.tpAmb),
          empId: cEmpId,
          chaveacesso: nFeEnt.infProt.chNFe,
          // Fields -> NFeEntInfoadic
          infadicInfcpl: nFeEnt.infAdic?.infCpl,
          //   infadicInfadfisco: nFeEnt.infAdic?.obsCont,  // Corrigir poie e array portane deve criar outro tabela
          logUserCad: userCurrentAuth.sub,
        },
      })
      // ============================================================== //
      //                   step 1.10: transaction                       //
      // ============================================================== //
      await this.prisma.$transaction([
        cNFeEnt,
        cNFeEntIde,
        cNFeEntEmit,
        lCadForn,
        cNFeEntDest,
        cNFeEntAutXml,
        ...cNFeEntItens,
        cNFeEntTransp,
        cNFeEntTotal,
        cNFeEntInfadic,
      ])
      // ============================================================== //
    } catch (error) {
      PrismaErrorHandler.handlePrismaError(error)
      throw new BadRequestException(`Erro ao gravar nFeEnt: ${error}`)
    }
  }

  // ###################################################################### //
  private async doCheckNFeEntCadastrada(
    nnf: string,
    serie: string,
    namb: string,
    empId: string,
    chaveacesso: string,
  ) {
    const cExitNFeEnt = await this.prisma.nFeEnt.findUnique({
      where: {
        nnf_serie_namb_emp_id_chaveacesso: {
          nnf: parseInt(nnf),
          serie: parseInt(serie),
          namb: parseInt(namb),
          empId,
          chaveacesso,
        },
      },
    })
    if (cExitNFeEnt) {
      throw new ConflictException('Nota Fiscal já cadastrada.')
    }
  }

  // ###################################################################### //
  private mapToNFeData(data: DataProps): NFeDataProps {
    //  console.log('det', data.nfeProc.NFe.infNFe.det)
    // console.log('vol', data.nfeProc.NFe.infNFe.transp.vol)

    const requiredNFeProperties = [
      'nfeProc',
      'nfeProc.NFe',
      'nfeProc.NFe.infNFe',
      'nfeProc.NFe.infNFe.ide',
      'nfeProc.NFe.infNFe.emit',
      'nfeProc.NFe.infNFe.emit.enderEmit',
      'nfeProc.NFe.infNFe.dest',
      'nfeProc.NFe.infNFe.dest.enderDest',
      'nfeProc.NFe.infNFe.autXML',
      'nfeProc.NFe.infNFe.det',
      'nfeProc.NFe.infNFe.transp',
      'nfeProc.NFe.infNFe.transp.transporta',
      'nfeProc.NFe.infNFe.transp.vol',
      'nfeProc.NFe.infNFe.total',
      'nfeProc.NFe.infNFe.total.ICMSTot',
      'nfeProc.NFe.infNFe.infAdic',
      'nfeProc.protNFe',
      'nfeProc.protNFe.infProt',
    ]

    const optionalNFeProperties = [
      'nfeProc.NFe.infNFe.avulsa',
      'nfeProc.NFe.infNFe.retirada',
      'nfeProc.NFe.infNFe.entrega',
    ]
    for (const property of requiredNFeProperties) {
      if (!hasNestedProperty(data, property)) {
        throw new ConflictException(
          `A propriedade ${property} está faltando no xml`,
        )
      }
    }

    for (const property of optionalNFeProperties) {
      if (!hasNestedProperty(data, property)) {
        //  console.warn(`A propriedade opcional ${property} está faltando no xml`)
      }
    }

    const infNFe = data.nfeProc.NFe.infNFe
    const ide = infNFe.ide
    const emit = infNFe.emit
    const dest = infNFe.dest
    const autXML = infNFe.autXML
    const det = infNFe.det
    const transp = infNFe.transp
    const total = infNFe.total
    const infAdic = infNFe.infAdic
    const infProt = data.nfeProc.protNFe.infProt

    return {
      ide,
      emit,
      dest,
      autXML,
      det,
      transp,
      total,
      infAdic,
      infProt,
    }
  }
}
// ########################################################################## //
