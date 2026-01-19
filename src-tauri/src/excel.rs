use anyhow::Result;
use rust_xlsxwriter::*;
use sqlx::SqlitePool;
use std::path::PathBuf;
use tauri::AppHandle;

pub struct ExcelLedger;

impl ExcelLedger {
    pub fn get_path(app: &AppHandle) -> Result<PathBuf> {
        let watiqa_dir = crate::utils::get_documents_dir(app);
        if !watiqa_dir.exists() {
            std::fs::create_dir_all(&watiqa_dir)?;
        }
        Ok(watiqa_dir.join("police_administrative.xlsx"))
    }

    pub async fn export_all(app: &AppHandle, pool: &SqlitePool) -> Result<()> {
        let path = Self::get_path(app)?;
        let mut workbook = Workbook::new();

        // 1. Commerces Sheet
        let sheet_commerces = workbook.add_worksheet();
        sheet_commerces.set_name("المحلات")?; // Arabic for Commerces

        let header_format = Format::new().set_bold().set_background_color(Color::Silver);

        let headers = [
            "ID",
            "الاسم",
            "العنوان",
            "المدينة",
            "الجماعة",
            "المسير",
            "البطاقة الوطنية",
            "الهاتف",
            "رقم الضريبة",
            "النشاط",
            "تاريخ الإنشاء",
        ];
        for (col, header) in headers.iter().enumerate() {
            sheet_commerces.write_with_format(0, col as u16, *header, &header_format)?;
        }

        let commerces = sqlx::query!(
            r#"SELECT id, name, address, city, commune, owner_name, cin, phone, patente, activity_type, created_at FROM commerces"#
        ).fetch_all(pool).await?;

        for (row_idx, commerce) in commerces.iter().enumerate() {
            let r = (row_idx + 1) as u32;
            sheet_commerces.write(r, 0, &commerce.id)?;
            sheet_commerces.write(r, 1, &commerce.name)?;
            sheet_commerces.write(
                r,
                2,
                commerce.address.as_ref().map(|s| s.as_str()).unwrap_or(""),
            )?;
            sheet_commerces.write(
                r,
                3,
                commerce.city.as_ref().map(|s| s.as_str()).unwrap_or(""),
            )?;
            sheet_commerces.write(
                r,
                4,
                commerce.commune.as_ref().map(|s| s.as_str()).unwrap_or(""),
            )?;
            sheet_commerces.write(
                r,
                5,
                commerce
                    .owner_name
                    .as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or(""),
            )?;
            sheet_commerces.write(
                r,
                6,
                commerce.cin.as_ref().map(|s| s.as_str()).unwrap_or(""),
            )?;
            sheet_commerces.write(
                r,
                7,
                commerce.phone.as_ref().map(|s| s.as_str()).unwrap_or(""),
            )?;
            sheet_commerces.write(
                r,
                8,
                commerce.patente.as_ref().map(|s| s.as_str()).unwrap_or(""),
            )?;
            sheet_commerces.write(
                r,
                9,
                commerce
                    .activity_type
                    .as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or(""),
            )?;
            if let Some(created) = &commerce.created_at {
                sheet_commerces.write(r, 10, created)?;
            }
        }

        // 2. Inspections Sheet
        let sheet_inspections = workbook.add_worksheet();
        sheet_inspections.set_name("المعاينات")?; // Arabic for Inspections

        let insp_headers = [
            "ID",
            "رقم المحضر",
            "المحل",
            "المفتش",
            "التاريخ",
            "الحالة",
            "الملخص",
        ];
        for (col, header) in insp_headers.iter().enumerate() {
            sheet_inspections.write_with_format(0, col as u16, *header, &header_format)?;
        }

        let inspections = sqlx::query!(
            r#"
            SELECT 
                i.id as "id!", 
                i.report_number as "report_number!", 
                c.name as "commerce_name!", 
                u.username as "inspector_name!", 
                i.inspection_date, 
                i.status as "status!", 
                i.summary 
            FROM inspections i
            JOIN commerces c ON i.commerce_id = c.id
            JOIN users u ON i.inspector_id = u.id
            "#
        )
        .fetch_all(pool)
        .await?;

        for (row_idx, insp) in inspections.iter().enumerate() {
            let r = (row_idx + 1) as u32;
            sheet_inspections.write(r, 0, &insp.id)?;
            sheet_inspections.write(r, 1, &insp.report_number)?;
            sheet_inspections.write(r, 2, &insp.commerce_name)?;
            sheet_inspections.write(r, 3, &insp.inspector_name)?;
            if let Some(date) = &insp.inspection_date {
                sheet_inspections.write(r, 4, date)?;
            }
            sheet_inspections.write(r, 5, &insp.status)?;
            sheet_inspections.write(
                r,
                6,
                insp.summary.as_ref().map(|s| s.as_str()).unwrap_or(""),
            )?;
        }

        workbook.save(&path)?;
        Ok(())
    }
}
