//! Gerador de DANFE profissional
//!
//! Layout moderno e elegante para NF-e usando printpdf

use printpdf::*;
use serde::{Deserialize, Serialize};
use std::io::BufWriter;

/// Dados para geração do DANFE
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DanfeInput {
    pub chave_acesso: String,
    pub numero: u32,
    pub serie: u16,
    pub data_emissao: String,
    pub natureza_operacao: String,
    pub protocolo: Option<String>,
    pub data_autorizacao: Option<String>,
    pub emitente: DanfeEmitente,
    pub destinatario: Option<DanfeDestinatario>,
    pub itens: Vec<DanfeItem>,
    pub totais: DanfeTotais,
    pub transporte: Option<DanfeTransporte>,
    pub informacoes_complementares: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DanfeEmitente {
    pub cnpj: String,
    pub razao_social: String,
    pub nome_fantasia: Option<String>,
    pub inscricao_estadual: Option<String>,
    pub endereco: String,
    pub municipio: String,
    pub uf: String,
    pub cep: String,
    pub telefone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DanfeDestinatario {
    pub cnpj_cpf: String,
    pub razao_social: String,
    pub inscricao_estadual: Option<String>,
    pub endereco: String,
    pub municipio: String,
    pub uf: String,
    pub cep: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DanfeItem {
    pub numero: u32,
    pub codigo: String,
    pub descricao: String,
    pub ncm: String,
    pub cfop: String,
    pub unidade: String,
    pub quantidade: f64,
    pub valor_unitario: f64,
    pub valor_total: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DanfeTotais {
    pub base_calculo_icms: f64,
    pub valor_icms: f64,
    pub base_calculo_st: f64,
    pub valor_st: f64,
    pub valor_produtos: f64,
    pub valor_frete: f64,
    pub valor_seguro: f64,
    pub valor_desconto: f64,
    pub valor_ipi: f64,
    pub valor_total: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DanfeTransporte {
    pub modalidade: String,
    pub transportadora: Option<String>,
    pub placa: Option<String>,
    pub uf: Option<String>,
}

/// Gera DANFE em PDF
pub fn gerar_danfe(input: &DanfeInput) -> Result<Vec<u8>, String> {
    // Criar documento A4 (210 x 297 mm)
    let (doc, page1, layer1) = PdfDocument::new(
        "DANFE - Documento Auxiliar da Nota Fiscal Eletrônica",
        Mm(210.0),
        Mm(297.0),
        "Página 1",
    );

    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Usar fonte built-in
    let font = doc.add_builtin_font(BuiltinFont::Helvetica)
        .map_err(|e| format!("Erro ao carregar fonte: {:?}", e))?;
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)
        .map_err(|e| format!("Erro ao carregar fonte bold: {:?}", e))?;

    let mut y: f32 = 280.0; // Começar do topo

    // === CABEÇALHO ===
    y = draw_header(&current_layer, &font, &font_bold, input, y)?;

    // === EMITENTE ===
    y = draw_emitente(&current_layer, &font, &font_bold, &input.emitente, y)?;

    // === DESTINATÁRIO ===
    if let Some(ref dest) = input.destinatario {
        y = draw_destinatario(&current_layer, &font, &font_bold, dest, y)?;
    }

    // === ITENS ===
    y = draw_itens(&current_layer, &font, &font_bold, &input.itens, y)?;

    // === TOTAIS ===
    y = draw_totais(&current_layer, &font, &font_bold, &input.totais, y)?;

    // === INFORMAÇÕES COMPLEMENTARES ===
    if let Some(ref info) = input.informacoes_complementares {
        let _ = draw_info_complementares(&current_layer, &font, &font_bold, info, y)?;
    }

    // === RODAPÉ ===
    draw_footer(&current_layer, &font, input)?;

    // Salvar PDF em buffer
    let mut buffer = Vec::new();
    {
        let mut writer = BufWriter::new(&mut buffer);
        doc.save(&mut writer)
            .map_err(|e| format!("Erro ao salvar PDF: {:?}", e))?;
    }

    Ok(buffer)
}

fn draw_header(
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    font_bold: &IndirectFontRef,
    input: &DanfeInput,
    mut y: f32,
) -> Result<f32, String> {
    // Título
    layer.use_text("DANFE", 18.0, Mm(90.0), Mm(y), font_bold);
    y -= 6.0;

    layer.use_text(
        "Documento Auxiliar da Nota Fiscal Eletrônica",
        8.0,
        Mm(65.0),
        Mm(y),
        font,
    );
    y -= 8.0;

    // Box com informações da NF-e
    draw_box(layer, 10.0, y - 30.0, 190.0, 35.0);

    // NF-e número, série
    let nfe_info = format!("NF-e Nº {:09} | Série {:03}", input.numero, input.serie);
    layer.use_text(&nfe_info, 12.0, Mm(15.0), Mm(y - 5.0), font_bold);
    y -= 10.0;

    // Data emissão
    layer.use_text(
        &format!("Emissão: {}", input.data_emissao),
        10.0,
        Mm(15.0),
        Mm(y - 5.0),
        font,
    );

    // Natureza da operação
    layer.use_text(
        &format!("Natureza: {}", input.natureza_operacao),
        10.0,
        Mm(100.0),
        Mm(y - 5.0),
        font,
    );
    y -= 10.0;

    // Chave de acesso
    layer.use_text("CHAVE DE ACESSO", 8.0, Mm(15.0), Mm(y - 5.0), font_bold);
    y -= 5.0;
    layer.use_text(&format_chave(&input.chave_acesso), 9.0, Mm(15.0), Mm(y - 5.0), font);
    y -= 10.0;

    // Protocolo
    if let Some(ref prot) = input.protocolo {
        layer.use_text(
            &format!(
                "Protocolo: {} - {}",
                prot,
                input.data_autorizacao.as_deref().unwrap_or("")
            ),
            8.0,
            Mm(15.0),
            Mm(y - 5.0),
            font,
        );
    }
    y -= 10.0;

    Ok(y)
}

fn draw_emitente(
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    font_bold: &IndirectFontRef,
    emit: &DanfeEmitente,
    mut y: f32,
) -> Result<f32, String> {
    // Título da seção
    draw_section_title(layer, font_bold, "EMITENTE", y);
    y -= 5.0;

    // Box
    draw_box(layer, 10.0, y - 30.0, 190.0, 35.0);

    // Razão Social
    layer.use_text(&emit.razao_social, 10.0, Mm(15.0), Mm(y - 5.0), font_bold);

    // CNPJ
    layer.use_text(
        &format!("CNPJ: {}", format_cnpj(&emit.cnpj)),
        9.0,
        Mm(140.0),
        Mm(y - 5.0),
        font,
    );
    y -= 8.0;

    // Endereço
    layer.use_text(&emit.endereco, 9.0, Mm(15.0), Mm(y - 5.0), font);
    y -= 6.0;

    // Cidade/UF/CEP
    layer.use_text(
        &format!("{} - {} - CEP: {}", emit.municipio, emit.uf, emit.cep),
        9.0,
        Mm(15.0),
        Mm(y - 5.0),
        font,
    );

    // IE
    layer.use_text(
        &format!("IE: {}", emit.inscricao_estadual.as_deref().unwrap_or("-")),
        9.0,
        Mm(140.0),
        Mm(y - 5.0),
        font,
    );
    y -= 6.0;

    // Telefone
    if let Some(ref tel) = emit.telefone {
        layer.use_text(&format!("Fone: {}", tel), 9.0, Mm(15.0), Mm(y - 5.0), font);
    }
    y -= 10.0;

    Ok(y)
}

fn draw_destinatario(
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    font_bold: &IndirectFontRef,
    dest: &DanfeDestinatario,
    mut y: f32,
) -> Result<f32, String> {
    draw_section_title(layer, font_bold, "DESTINATÁRIO / REMETENTE", y);
    y -= 5.0;

    draw_box(layer, 10.0, y - 25.0, 190.0, 30.0);

    // Razão Social
    layer.use_text(&dest.razao_social, 10.0, Mm(15.0), Mm(y - 5.0), font_bold);

    // CNPJ/CPF
    layer.use_text(
        &format!("CNPJ/CPF: {}", format_cnpj_cpf(&dest.cnpj_cpf)),
        9.0,
        Mm(140.0),
        Mm(y - 5.0),
        font,
    );
    y -= 8.0;

    // Endereço
    layer.use_text(&dest.endereco, 9.0, Mm(15.0), Mm(y - 5.0), font);
    y -= 6.0;

    // Cidade/UF/CEP
    layer.use_text(
        &format!("{} - {} - CEP: {}", dest.municipio, dest.uf, dest.cep),
        9.0,
        Mm(15.0),
        Mm(y - 5.0),
        font,
    );

    // IE
    layer.use_text(
        &format!("IE: {}", dest.inscricao_estadual.as_deref().unwrap_or("-")),
        9.0,
        Mm(140.0),
        Mm(y - 5.0),
        font,
    );
    y -= 15.0;

    Ok(y)
}

fn draw_itens(
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    font_bold: &IndirectFontRef,
    itens: &[DanfeItem],
    mut y: f32,
) -> Result<f32, String> {
    draw_section_title(layer, font_bold, "PRODUTOS / SERVIÇOS", y);
    y -= 5.0;

    // Cabeçalho da tabela
    let headers = ["Cód.", "Descrição", "NCM", "CFOP", "Un", "Qtd", "V.Unit", "V.Total"];
    let x_positions: [f32; 8] = [15.0, 35.0, 100.0, 125.0, 145.0, 160.0, 175.0, 190.0];

    for (i, header) in headers.iter().enumerate() {
        layer.use_text(*header, 7.0, Mm(x_positions[i]), Mm(y - 3.0), font_bold);
    }
    y -= 5.0;

    // Linha separadora
    draw_line(layer, 10.0, y, 200.0, y);
    y -= 3.0;

    // Itens (limitar a 20 por página)
    for item in itens.iter().take(20) {
        layer.use_text(&item.codigo, 7.0, Mm(15.0), Mm(y), font);

        // Descrição truncada
        let desc = if item.descricao.len() > 40 {
            format!("{}...", &item.descricao[..37])
        } else {
            item.descricao.clone()
        };
        layer.use_text(&desc, 7.0, Mm(35.0), Mm(y), font);

        layer.use_text(&item.ncm, 7.0, Mm(100.0), Mm(y), font);
        layer.use_text(&item.cfop, 7.0, Mm(125.0), Mm(y), font);
        layer.use_text(&item.unidade, 7.0, Mm(145.0), Mm(y), font);
        layer.use_text(&format!("{:.2}", item.quantidade), 7.0, Mm(160.0), Mm(y), font);
        layer.use_text(&format!("{:.2}", item.valor_unitario), 7.0, Mm(175.0), Mm(y), font);
        layer.use_text(&format!("{:.2}", item.valor_total), 7.0, Mm(190.0), Mm(y), font);

        y -= 5.0;
    }

    y -= 5.0;
    Ok(y)
}

fn draw_totais(
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    font_bold: &IndirectFontRef,
    totais: &DanfeTotais,
    mut y: f32,
) -> Result<f32, String> {
    draw_section_title(layer, font_bold, "CÁLCULO DO IMPOSTO", y);
    y -= 5.0;

    draw_box(layer, 10.0, y - 25.0, 190.0, 30.0);

    // Linha 1
    layer.use_text(
        &format!("Base ICMS: R$ {:.2}", totais.base_calculo_icms),
        8.0,
        Mm(15.0),
        Mm(y - 5.0),
        font,
    );
    layer.use_text(
        &format!("Valor ICMS: R$ {:.2}", totais.valor_icms),
        8.0,
        Mm(60.0),
        Mm(y - 5.0),
        font,
    );
    layer.use_text(
        &format!("Base ST: R$ {:.2}", totais.base_calculo_st),
        8.0,
        Mm(110.0),
        Mm(y - 5.0),
        font,
    );
    layer.use_text(
        &format!("Valor ST: R$ {:.2}", totais.valor_st),
        8.0,
        Mm(155.0),
        Mm(y - 5.0),
        font,
    );
    y -= 8.0;

    // Linha 2
    layer.use_text(
        &format!("V. Produtos: R$ {:.2}", totais.valor_produtos),
        8.0,
        Mm(15.0),
        Mm(y - 5.0),
        font,
    );
    layer.use_text(
        &format!("V. Frete: R$ {:.2}", totais.valor_frete),
        8.0,
        Mm(60.0),
        Mm(y - 5.0),
        font,
    );
    layer.use_text(
        &format!("V. Desconto: R$ {:.2}", totais.valor_desconto),
        8.0,
        Mm(110.0),
        Mm(y - 5.0),
        font,
    );
    layer.use_text(
        &format!("V. IPI: R$ {:.2}", totais.valor_ipi),
        8.0,
        Mm(155.0),
        Mm(y - 5.0),
        font,
    );
    y -= 10.0;

    // Total em destaque
    layer.use_text(
        &format!("VALOR TOTAL DA NOTA: R$ {:.2}", totais.valor_total),
        12.0,
        Mm(120.0),
        Mm(y - 5.0),
        font_bold,
    );
    y -= 15.0;

    Ok(y)
}

fn draw_info_complementares(
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    font_bold: &IndirectFontRef,
    info: &str,
    mut y: f32,
) -> Result<f32, String> {
    draw_section_title(layer, font_bold, "INFORMAÇÕES COMPLEMENTARES", y);
    y -= 5.0;

    draw_box(layer, 10.0, y - 20.0, 190.0, 25.0);

    // Quebrar texto em linhas
    let max_chars = 100;
    let mut current_y = y - 5.0;
    for chunk in info.chars().collect::<Vec<_>>().chunks(max_chars) {
        let line: String = chunk.iter().collect();
        layer.use_text(&line, 7.0, Mm(15.0), Mm(current_y), font);
        current_y -= 4.0;
    }

    y -= 25.0;
    Ok(y)
}

fn draw_footer(
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    input: &DanfeInput,
) -> Result<(), String> {
    let y: f32 = 15.0;

    layer.use_text(
        "Consulte a autenticidade em: www.nfe.fazenda.gov.br/portal",
        8.0,
        Mm(50.0),
        Mm(y),
        font,
    );

    layer.use_text(
        &format!("Chave: {}", format_chave(&input.chave_acesso)),
        7.0,
        Mm(40.0),
        Mm(y - 5.0),
        font,
    );

    Ok(())
}

// === Funções auxiliares de desenho ===

fn draw_box(layer: &PdfLayerReference, x: f32, y: f32, width: f32, height: f32) {
    let points = vec![
        (Point::new(Mm(x), Mm(y)), false),
        (Point::new(Mm(x + width), Mm(y)), false),
        (Point::new(Mm(x + width), Mm(y + height)), false),
        (Point::new(Mm(x), Mm(y + height)), false),
    ];

    let line = Line {
        points,
        is_closed: true,
    };

    layer.set_outline_color(Color::Rgb(Rgb::new(0.7, 0.7, 0.7, None)));
    layer.set_outline_thickness(0.5);
    layer.add_line(line);
}

fn draw_line(layer: &PdfLayerReference, x1: f32, y1: f32, x2: f32, y2: f32) {
    let points = vec![
        (Point::new(Mm(x1), Mm(y1)), false),
        (Point::new(Mm(x2), Mm(y2)), false),
    ];

    let line = Line {
        points,
        is_closed: false,
    };

    layer.set_outline_color(Color::Rgb(Rgb::new(0.8, 0.8, 0.8, None)));
    layer.set_outline_thickness(0.3);
    layer.add_line(line);
}

fn draw_section_title(layer: &PdfLayerReference, font: &IndirectFontRef, title: &str, y: f32) {
    layer.set_fill_color(Color::Rgb(Rgb::new(0.2, 0.2, 0.2, None)));
    layer.use_text(title, 9.0, Mm(10.0), Mm(y), font);
}

// === Funções de formatação ===

fn format_chave(chave: &str) -> String {
    let digits: String = chave.chars().filter(|c| c.is_ascii_digit()).collect();
    digits
        .chars()
        .collect::<Vec<_>>()
        .chunks(4)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}

fn format_cnpj(cnpj: &str) -> String {
    let digits: String = cnpj.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() == 14 {
        format!(
            "{}.{}.{}/{}-{}",
            &digits[0..2],
            &digits[2..5],
            &digits[5..8],
            &digits[8..12],
            &digits[12..14]
        )
    } else {
        cnpj.to_string()
    }
}

fn format_cnpj_cpf(doc: &str) -> String {
    let digits: String = doc.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() == 14 {
        format_cnpj(&digits)
    } else if digits.len() == 11 {
        format!(
            "{}.{}.{}-{}",
            &digits[0..3],
            &digits[3..6],
            &digits[6..9],
            &digits[9..11]
        )
    } else {
        doc.to_string()
    }
}
