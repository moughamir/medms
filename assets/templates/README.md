# Watiqa-Link Document Templates Documentation

All `.docx` templates should be placed in `assets/templates/`.

## Placeholder Syntax
The template engine uses `{{PLACEHOLDER}}` syntax. It is robust against XML fragmentation (e.g., if Word splits the text into multiple `<w:t>` tags).

## Available Placeholders

### Administrative Header
*   `{{COUNTRY}}`: المملكة المغربية
*   `{{MINISTRY}}`: وزارة الداخلية
*   `{{PROVINCE}}`: إقليم النواصر
*   `{{COMMUNE}}`: جماعة بوسكورة
*   `{{DEPARTMENT}}`: قسم الشرطة الإدارية

### Document Core
*   `{{TIMESTAMP}}`: تاريخ المعاينة (DD/MM/YYYY)
*   `{{UUID}}`: الرقم الفريد للمستند
*   `{{REPORT_NUMBER}}`: رقم المحضر

### Agent Information
*   `{{AGENT_NAME}}`: اسم المفتش
*   `{{AGENT_GRADE}}`: درجة المفتش

### Citizen / Establishment Information
*   `{{CITIZEN_NAME}}`: اسم صاحب المحل / المسير
*   `{{CITIZEN_CIN}}`: رقم البطاقة الوطنية
*   `{{CITIZEN_ADDRESS}}`: العنوان
*   `{{COMMERCE_NAME}}`: اسم المحل التجاري

## Sample Template Layout (Administrative Police)

```text
{{COUNTRY}}
{{MINISTRY}}
{{PROVINCE}}
{{COMMUNE}}
{{DEPARTMENT}}

محضر معاينة رقم: {{REPORT_NUMBER}}

بناء على القانون رقم...
نحن الموقعون أسفله {{AGENT_NAME}}، بصفة {{AGENT_GRADE}}...

قمنا بمعاينة المحل المسمى {{CITIZEN_NAME}} الحامل للبطاقة الوطنية رقم {{CITIZEN_CIN}}...

الملاحظات:
{{SUMMARY}}

التوقيع:
{{AGENT_NAME}}
```
