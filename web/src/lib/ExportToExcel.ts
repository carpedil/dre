import * as XLSX from 'xlsx';

export const ExportToExcel = (data: any, file_name: string) => {
	// 生成 worksheet
	const worksheet = XLSX.utils.json_to_sheet(data);

	// 导出 Excel
	const workbook = XLSX.utils.book_new();
	XLSX.utils.book_append_sheet(workbook, worksheet, 'Sheet1');

	// 触发下载
	const excelBuffer = XLSX.write(workbook, { bookType: 'xlsx', type: 'array' });
	const excelBlob = new Blob([excelBuffer], { type: 'application/octet-stream' });
	const link = document.createElement('a');
	link.href = URL.createObjectURL(excelBlob);

	const datetime_str = new Date().toLocaleString('zh-CN', {
		year: 'numeric',
		month: '2-digit',
		day: '2-digit',
		hour: '2-digit',
		minute: '2-digit',
		second: '2-digit'
	});
	link.download = `${file_name}_${datetime_str}.xlsx`;
	link.click();
};
