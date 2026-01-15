//! Arabic document templates for administrative documents

use crate::models::{CommerceInfo, InspectionRecord};

// ==================== Document Headers ====================

/// Header for Procès-Verbal (محضر معاينة)
pub fn pv_header() -> String {
    "محضر معاينة\n\
    ـــــــــــــــــــــــــــــــــــــــــــــــــــــــــ\n\n"
        .to_string()
}

/// Header for Avertissement (إنذار)
pub fn warning_header() -> String {
    "إنـــــــذار\n\
    ـــــــــــــــــــــــــــــــــــــــــــــــــــــــــ\n\n\
    بناء على المعاينة المنجزة من طرف مصلحة الشرطة الإدارية،\n"
        .to_string()
}

/// Header for Mise en Demeure (إعذار)
pub fn notice_header() -> String {
    "إعـــــذار\n\
    ـــــــــــــــــــــــــــــــــــــــــــــــــــــــــ\n\n\
    نظرا لعدم الامتثال للإنذار السابق الموجه إليكم،\n\
    وبناء على مقتضيات القانون المعمول به،\n"
        .to_string()
}

/// Header for Décision de Fermeture (قرار غلق)
pub fn closure_header() -> String {
    "قرار غلق إداري\n\
    ـــــــــــــــــــــــــــــــــــــــــــــــــــــــــ\n\n\
    بناء على عدم الامتثال للإعذارات والإنذارات السابقة،\n\
    وتطبيقا للقانون المعمول به،\n\n\
    يقرر ما يلي:\n"
        .to_string()
}

/// Header for Amende (غرامة)
pub fn fine_header() -> String {
    "غرامة إدارية\n\
    ـــــــــــــــــــــــــــــــــــــــــــــــــــــــــ\n\n\
    تطبيقا للقانون المعمول به في مجال ضبط المخالفات الإدارية،\n"
        .to_string()
}

/// Header for Rapport de Contrôle (تقرير مراقبة)
pub fn report_header() -> String {
    "تقرير مراقبة\n\
    ـــــــــــــــــــــــــــــــــــــــــــــــــــــــــ\n\n\
    في إطار المهام المنوطة بمصلحة الشرطة الإدارية،\n\
    تم القيام بزيارة مراقبة للمحل التالي:\n"
        .to_string()
}

/// Header for Convocation (استدعاء)
pub fn convocation_header() -> String {
    "استدعـــاء\n\
    ـــــــــــــــــــــــــــــــــــــــــــــــــــــــــ\n\n\
    في إطار معالجة الملف المتعلق بالمخالفات المرتكبة،\n"
        .to_string()
}

// ==================== Content Sections ====================

/// Commerce information section
pub fn commerce_section(commerce: &CommerceInfo) -> String {
    format!(
        "\nمعلومات المحل التجاري:\n\
        ـــــــــــــــــــــــــــــــــــــــــ\n\
        التسمية التجارية: {}\n\
        صاحب المحل: {}\n\
        رقم البطاقة الوطنية: {}\n\
        رقم الهاتف: {}\n\
        عنوان المحل: {}\n\
        الحي: {}\n\
        نوع النشاط: {}\n\
        رقم ICE: {}\n\
        الرخصة: {}\n\n",
        commerce.denomination,
        commerce.owner_name,
        commerce.owner_cin,
        commerce.owner_phone,
        commerce.establishment_address,
        commerce.quartier,
        commerce.activity_type,
        commerce.ice_number,
        if commerce.has_autorisation {
            format!("نعم - رقم {}", commerce.autorisation_number)
        } else {
            "لا".to_string()
        }
    )
}

/// Addressee section for letters
pub fn addressee_section(commerce: &CommerceInfo) -> String {
    format!(
        "إلى السيد(ة): {}\n\
        صاحب(ة) المحل التجاري: {}\n\
        الكائن بـ: {}\n\
        الحي: {}",
        commerce.owner_name,
        commerce.denomination,
        commerce.establishment_address,
        commerce.quartier
    )
}

/// Inspection details section
pub fn inspection_section(inspection: &InspectionRecord) -> String {
    format!(
        "\nتفاصيل المعاينة:\n\
        ـــــــــــــــــــــــــــــــــــــــــ\n\
        تاريخ المعاينة: {} على الساعة {}\n\
        العون المكلف: {} - {}\n\n\
        المخالفة المعاينة:\n{}\n\n\
        المرجع القانوني: {}\n\n\
        الإجراءات المتخذة:\n{}\n",
        inspection.inspection_date,
        inspection.inspection_time,
        inspection.inspector_name,
        inspection.inspector_grade,
        inspection.description,
        inspection.legal_reference,
        inspection.measures_taken
    )
}

/// Signature section
pub fn signature_section(inspector_name: &str, inspector_grade: &str) -> String {
    format!(
        "\n\nـــــــــــــــــــــــــــــــــــــــــ\n\
        {} - {}\n\
        التوقيع والختم\n",
        inspector_name, inspector_grade
    )
}

// ==================== Document Footers ====================

/// Footer for Avertissement
pub fn warning_footer() -> String {
    "في حالة عدم الامتثال، ستتخذ في حقكم الإجراءات القانونية اللازمة.\n\n\
    وحرر هذا الإنذار للإشعار والعمل بموجبه.\n"
        .to_string()
}

/// Footer for Mise en Demeure
pub fn notice_footer() -> String {
    "في حالة عدم الامتثال داخل الأجل المحدد، سيتم اتخاذ قرار الغلق الإداري و/أو فرض غرامات مالية.\n\n\
    وحرر هذا الإعذار للإشعار والعمل بموجبه قبل فوات الأجل.\n"
        .to_string()
}

/// Footer for Décision de Fermeture
pub fn closure_footer() -> String {
    "يسري مفعول هذا القرار ابتداء من تاريخ التبليغ.\n\n\
    لصاحب المحل الحق في الطعن في هذا القرار أمام الجهات المختصة داخل الآجال القانونية.\n"
        .to_string()
}

/// Footer for Amende
pub fn fine_footer() -> String {
    "في حالة عدم الأداء داخل الأجل المحدد:\n\
    - ستضاف إلى المبلغ الأصلي علاوة التأخير.\n\
    - سيتم اللجوء إلى التحصيل الجبري طبقا للقانون.\n\n\
    يمكنكم أداء هذه الغرامة لدى قابض الجماعة.\n"
        .to_string()
}

/// Footer for Convocation
pub fn convocation_footer() -> String {
    "يتعين عليكم الحضور شخصيا أو بواسطة وكيل قانوني.\n\n\
    في حالة عدم الحضور، ستتخذ في حقكم الإجراءات اللازمة.\n"
        .to_string()
}
