use crate::gui::text_utils::reshape;
use crate::gui::views::settings::Language;
use std::collections::HashMap;
use std::sync::OnceLock;

/// Global dictionary for translations
static DICTIONARY: OnceLock<HashMap<&'static str, Translation>> = OnceLock::new();

struct Translation {
    arabic: &'static str,
    french: &'static str,
}

impl Translation {
    const fn new(arabic: &'static str, french: &'static str) -> Self {
        Self { arabic, french }
    }
}

/// Initialize the dictionary
fn get_dictionary() -> &'static HashMap<&'static str, Translation> {
    DICTIONARY.get_or_init(|| {
        let mut m = HashMap::new();
        
        // Navigation & Titles
        m.insert("app_title", Translation::new("الشرطة الإدارية", "Police Administrative"));
        m.insert("dashboard", Translation::new("لوحة التحكم", "Tableau de bord"));
        m.insert("new_commerce", Translation::new("محل جديد", "Nouveau commerce"));
        m.insert("new_inspection", Translation::new("معاينة جديدة", "Nouvelle inspection"));
        m.insert("new_document", Translation::new("وثيقة جديدة", "Nouveau document"));
        m.insert("documents_registry", Translation::new("سجل الوثائق", "Registre des documents"));
        m.insert("settings", Translation::new("الإعدادات", "Paramètres"));
        m.insert("login", Translation::new("تسجيل الدخول", "Connexion"));
        m.insert("password", Translation::new("كلمة السر", "Mot de passe"));
        m.insert("enter", Translation::new("دخول", "Entrer"));
        m.insert("incorrect_password", Translation::new("كلمة السر غير صحيحة", "Mot de passe incorrect"));

        // Common Actions
        m.insert("save", Translation::new("حفظ", "Enregistrer"));
        m.insert("clear", Translation::new("مسح", "Effacer"));
        m.insert("search", Translation::new("بحث", "Rechercher"));
        m.insert("refresh", Translation::new("تحديث", "Actualiser"));
        m.insert("updated", Translation::new("تم التحديث", "Actualisé"));
        m.insert("error", Translation::new("خطأ", "Erreur"));

        // Dashboard Stats
        m.insert("commerces", Translation::new("المحلات", "Commerces"));
        m.insert("inspections", Translation::new("المعاينات", "Inspections"));
        m.insert("documents", Translation::new("الوثائق", "Documents"));
        m.insert("no_license", Translation::new("بدون رخصة", "Sans autorisation"));
        m.insert("fines_issued", Translation::new("الغرامات الصادرة", "Amendes émises"));
        m.insert("collected", Translation::new("المحصلة", "Perçues"));
        m.insert("pending", Translation::new("المستحقة", "Dues"));
        m.insert("overdue", Translation::new("متأخرة", "En retard"));
        m.insert("violations_by_type", Translation::new("المخالفات حسب النوع", "Infractions par type"));

        // Forms
        m.insert("add_commerce", Translation::new("إضافة محل جديد", "Ajouter un nouveau commerce"));
        m.insert("trade_name", Translation::new("التسمية التجارية", "Dénomination commerciale"));
        m.insert("owner_name", Translation::new("اسم المالك", "Nom du propriétaire"));
        m.insert("cin", Translation::new("رقم ب.و.ت", "C.I.N"));
        m.insert("phone", Translation::new("الهاتف", "Téléphone"));
        m.insert("owner_address", Translation::new("عنوان المالك", "Adresse du propriétaire"));
        m.insert("shop_address", Translation::new("عنوان المحل", "Adresse du commerce"));
        m.insert("district", Translation::new("الحي", "Quartier"));
        m.insert("activity", Translation::new("نوع النشاط", "Type d'activité"));
        m.insert("ice", Translation::new("رقم ICE", "N° ICE"));
        m.insert("patente", Translation::new("رقم الباطونطا", "N° Patente"));
        m.insert("has_license", Translation::new("رخصة", "Autorisation"));
        m.insert("yes", Translation::new("نعم", "Oui"));
        m.insert("license_num", Translation::new("رقم الرخصة", "N° Autorisation"));
        m.insert("license_date", Translation::new("تاريخ الرخصة", "Date d'autorisation"));
        m.insert("commerce_saved", Translation::new("تم حفظ المحل بنجاح", "Commerce enregistré avec succès"));
        
        m.insert("inspector_name", Translation::new("اسم العون", "Nom de l'agent"));
        m.insert("inspector_grade", Translation::new("رتبة العون", "Grade de l'agent"));
        m.insert("shop", Translation::new("المحل", "Commerce"));
        m.insert("violation_type", Translation::new("نوع المخالفة", "Type d'infraction"));
        m.insert("description", Translation::new("وصف المخالفة", "Description"));
        m.insert("measures", Translation::new("الإجراءات المتخذة", "Mesures prises"));
        m.insert("inspection_saved", Translation::new("تم حفظ المعاينة بنجاح", "Inspection enregistrée avec succès"));

        m.insert("choose_type", Translation::new("اختر نوع الوثيقة", "Choisir le type de document"));
        m.insert("create", Translation::new("إنشاء", "Créer"));
        m.insert("loading_error", Translation::new("خطأ في تحميل الوثائق", "Erreur lors du chargement des documents"));
        m.insert("type", Translation::new("النوع", "Type"));
        m.insert("select_type", Translation::new("اختر النوع", "Sélectionner le type"));
        m.insert("content", Translation::new("المحتوى / الملاحظات", "Contenu / Remarques"));
        m.insert("doc_created", Translation::new("تم إنشاء الوثيقة", "Document créé"));
        m.insert("open_folder", Translation::new("فتح المجلد", "Ouvrir le dossier"));
        m.insert("open", Translation::new("فتح", "Ouvrir"));

        m.insert("commune", Translation::new("الجماعة", "Commune"));
        m.insert("arrondissement", Translation::new("المقاطعة", "Arrondissement"));
        m.insert("db_file", Translation::new("ملف قاعدة البيانات", "Fichier de base de données"));
        m.insert("settings_saved", Translation::new("تم حفظ الإعدادات", "Paramètres enregistrés"));

        // Onboarding
        m.insert("back", Translation::new("السابق", "Retour"));
        m.insert("next", Translation::new("التالي", "Suivant"));
        m.insert("finish", Translation::new("إنهاء", "Terminer"));
        m.insert("location_settings", Translation::new("إعدادات الموقع", "Paramètres de localisation"));
        m.insert("identity_settings", Translation::new("بيانات العون", "Informations de l'agent"));
        m.insert("database_settings", Translation::new("قاعدة البيانات", "Base de données"));
        
        m
    })
}

/// Translates a key based on the selected language.
/// Returns "RESHAPED_ARABIC / FRENCH" for ArabicFrench mode.
pub fn tr(key: &str, lang: &Language) -> String {
    let dict = get_dictionary();
    
    if let Some(translation) = dict.get(key) {
        match lang {
            Language::Arabic => reshape(translation.arabic),
            Language::French => translation.french.to_string(),
            Language::ArabicFrench => format!("{} / {}", reshape(translation.arabic), translation.french),
        }
    } else {
        // Fallback or missing key
        format!("MISSING:{}", key)
    }
}

/// Translates a dynamic text (where we can't look up a static key easily, or for keys that need extra context).
/// This helper is for when you have the raw Arabic/French strings and just want to format them according to mode.
/// This avoids needing a perfect dictionary for every single dynamic string if they are generated elsewhere.
pub fn tr_dynamic(arabic: &str, french: &str, lang: &Language) -> String {
    match lang {
        Language::Arabic => reshape(arabic),
        Language::French => french.to_string(),
        Language::ArabicFrench => format!("{} / {}", reshape(arabic), french),
    }
}
