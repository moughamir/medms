use docx_rs::*;
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let template_dir = std::path::Path::new("../assets/templates");
    if !template_dir.exists() {
        std::fs::create_dir_all(template_dir)?;
    }

    create_pv_inspection(template_dir.join("pv_inspection.docx"))?;
    create_mise_en_demeure(template_dir.join("mise_en_demeure.docx"))?;
    create_decision_fermeture(template_dir.join("decision_fermeture.docx"))?;

    println!("Administrative templates generated successfully in assets/templates/");
    Ok(())
}

fn create_pv_inspection(path: std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(path)?;
    Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("{{COUNTRY}}")).align(AlignmentType::Right))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("{{MINISTRY}}")).align(AlignmentType::Right))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("{{PROVINCE}}")).align(AlignmentType::Right))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("{{COMMUNE}}")).align(AlignmentType::Right))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("{{DEPARTMENT}}")).align(AlignmentType::Right))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("")),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("محضر معاينة رقم: {{REPORT_NUMBER}}").bold())
                .align(AlignmentType::Center),
        )
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("")),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("بناء على القانون التنظيمي رقم 113.14 المتعلق بالجماعات ولاسيما المواد 100 و 106 منه؛"))
                .align(AlignmentType::Right),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("بناء على الظهير الشريف رقم 1.15.85 الصادر في 20 من رمضان 1436 (7 يوليو 2015) بتنفيذ القانون التنظيمي رقم 113.14؛"))
                .align(AlignmentType::Right),
        )
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("")),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("نحن الموقعون أسفله {{AGENT_NAME}}، بصفة {{AGENT_GRADE}}، انتقلنا بتاريخ {{TIMESTAMP}} إلى المحل المسمى:"))
                .align(AlignmentType::Right),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("الاسم/الشعار: {{COMMERCE_NAME}}").bold())
                .align(AlignmentType::Right),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("المسير: {{CITIZEN_NAME}}"))
                .align(AlignmentType::Right),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("العنوان: {{CITIZEN_ADDRESS}}"))
                .align(AlignmentType::Right),
        )
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("")),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("الملاحظات والمعاينات:").bold())
                .align(AlignmentType::Right),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("{{SUMMARY}}"))
                .align(AlignmentType::Right),
        )
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("")),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("توقيع المفتش:"))
                .align(AlignmentType::Left),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("{{AGENT_NAME}}"))
                .align(AlignmentType::Left),
        )
        .build()
        .pack(file)?;
    Ok(())
}

fn create_mise_en_demeure(path: std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(path)?;
    Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("{{COUNTRY}}")).align(AlignmentType::Right))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("{{MINISTRY}}")).align(AlignmentType::Right))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("{{PROVINCE}}")).align(AlignmentType::Right))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("{{COMMUNE}}")).align(AlignmentType::Right))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("{{DEPARTMENT}}")).align(AlignmentType::Right))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("")),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("إنذار / إعذار").bold())
                .align(AlignmentType::Center),
        )
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("")),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("إلى السيد(ة): {{CITIZEN_NAME}}"))
                .align(AlignmentType::Right),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("بصفتكم مسير لـ: {{COMMERCE_NAME}}"))
                .align(AlignmentType::Right),
        )
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("")),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("علاقة بمحضر المعاينة المنجز بتاريخ {{TIMESTAMP}} والذي سجل مجموعة من المخالفات، نطلب منكم تسوية وضعية محلكم داخل أجل لا يتعدى 48 ساعة."))
                .align(AlignmentType::Right),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("وفي حالة عدم الامتثال، ستضطر السلطات المعنية لاتخاذ الإجراءات الزجرية المعمول بها."))
                .align(AlignmentType::Right),
        )
        .build()
        .pack(file)?;
    Ok(())
}

fn create_decision_fermeture(path: std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(path)?;
    Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("{{COUNTRY}}")).align(AlignmentType::Right))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("{{MINISTRY}}")).align(AlignmentType::Right))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("{{PROVINCE}}")).align(AlignmentType::Right))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("{{COMMUNE}}")).align(AlignmentType::Right))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("{{DEPARTMENT}}")).align(AlignmentType::Right))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("")),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("قرار إداري بالإغلاق").bold())
                .align(AlignmentType::Center),
        )
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("")),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("يقرر ما يلي:"))
                .align(AlignmentType::Right),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("المادة الأولى: يغلق المحل التجاري المسمى {{COMMERCE_NAME}} الكائن بـ {{CITIZEN_ADDRESS}}."))
                .align(AlignmentType::Right),
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("المادة الثانية: يعهد بتنفيذ هذا القرار إلى السلطات المحلية والمصالح الأمنية المختصة."))
                .align(AlignmentType::Right),
        )
        .build()
        .pack(file)?;
    Ok(())
}
