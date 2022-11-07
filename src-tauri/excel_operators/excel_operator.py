import sys

import openpyxl
import xlrd
from datetime import datetime


def write_basic_excel(export_datas, output):
    valid_rows = {}
    for row_data in export_datas:
        if row_data[0].startswith('5'):
            num, title, _, _, current_debit, _, ending_debit, _ = row_data
            valid_rows[num] = {
                "title": title,
                'current_debit': float(str(current_debit).replace(',', '')) if current_debit else 0,
                "ending_debit": float(str(ending_debit).replace(',', '')) if ending_debit else 0
            }
    template_wb = openpyxl.load_workbook('basic/result_template.xlsx')
    template_ws = template_wb['Sheet1']
    for row_index, row_data in enumerate(template_ws):
        for col_index, cell in enumerate(row_data):
            if isinstance(cell.value, str) or isinstance(cell.value, int):
                cell_value = str(cell.value)
                try:
                    if "+" in cell_value and cell_value.startswith('5'):
                        total_list = [valid_rows[seg]['ending_debit'] for seg in cell_value.split("+")]
                        total = sum(total_list)
                        cell.value = total
                    elif cell_value.startswith('5'):
                        cell.value = valid_rows[cell_value]['ending_debit']
                except Exception as e:
                    print(e.args[0])
                    print(cell.value)
            elif cell.value:
                print(cell.value, type(cell.value))
    template_wb.save(output)


operator_matches = {
    '总账余额表': write_basic_excel,
}


def read_export_excel(file_path: str):
    path, extension = file_path.split('.')
    output_path = f"{path}_{datetime.now().strftime('%Y%m%d%H%M%S')}.{extension}"
    workbook = xlrd.open_workbook(file_path)
    sheet = workbook.sheets()[0]
    rows_count = sheet.nrows
    title = sheet.row_values(0)[0]
    sheet_data = [sheet.row_values(row_index) for row_index in range(rows_count)]
    for title_start, operate in operator_matches.items():
        if title.startswith(title_start):
            operate(sheet_data, output_path)


def write_project_excel(export_datas):
    print(export_datas)


if __name__ == "__main__":
    #     sample_file_path = 'basic/sample_export.xls'
    #     result = read_export_excel(file_path=sample_file_path)
    #     print(result)
    file_path = sys.argv[1]
    result = read_export_excel(file_path)
    print(file_path, result)
