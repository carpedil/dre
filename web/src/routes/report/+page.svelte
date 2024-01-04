<script lang="ts">
	import type { ApiMessageTestCase, ApiMessageTestRecordsSummary } from '$lib';
	import { ExportToExcel } from '$lib/ExportToExcel';
	import { getApiMessageTestRecordsSummaryList } from '$lib/api/apimessage';
	import { update_what_you_focus_on } from '$lib/stores';
	import { onMount } from 'svelte';
	// import * as XLSX from 'xlsx';

	let reportData: ApiMessageTestRecordsSummary[] = [];
	let highlightedRowIndex: number;
	let highlightedSrvId: number;
	let totalTestCount: number = 0;

	const highlightRow = async (rowIndex: number, srv_name: string, srv_id: number) => {
		highlightedRowIndex = rowIndex;
		highlightedSrvId = srv_id;
		expandNestedData(rowIndex);
	};

	const expandNestedData = (rowIndex: number) => {
		const row = document.getElementById(`nested-row-${rowIndex}`);
		if (row?.classList.contains('hidden')) {
			row?.classList.remove('hidden');
		} else {
			row?.classList.add('hidden');
		}
	};

	const setApiId = (id: number) => {
		update_what_you_focus_on(highlightedSrvId, id);
	};

	const download = () => {
		console.log('download');

		// 数据准备
		let data: ApiMessageTestCase[] = [];
		reportData.forEach((item: ApiMessageTestRecordsSummary) => {
			data = [...data, ...item.test_cases_summary];
		});

		// // 生成 worksheet
		// const worksheet = XLSX.utils.json_to_sheet(data);

		// // 导出 Excel
		// const workbook = XLSX.utils.book_new();
		// XLSX.utils.book_append_sheet(workbook, worksheet, 'Sheet1');

		// // 触发下载
		// const excelBuffer = XLSX.write(workbook, { bookType: 'xlsx', type: 'array' });
		// const excelBlob = new Blob([excelBuffer], { type: 'application/octet-stream' });
		// const link = document.createElement('a');
		// link.href = URL.createObjectURL(excelBlob);
		// link.download = 'test_case.xlsx';
		// link.click();
		ExportToExcel(data, 'ApiMessageTestRecordsSummary');
	};

	onMount(async () => {
		reportData = await getApiMessageTestRecordsSummaryList();
		reportData.forEach((item: ApiMessageTestRecordsSummary) => {
			totalTestCount += item.tested;
		});
	});
</script>

<div>
	<table class="w-full border border-separate text-xs">
		<tr class="bg-white text-left sticky top-10">
			<th class="w-[9vw] text-center">SERVER NAME</th>
			<th class="w-[4vw] text-center">P1</th>
			<th class="w-[4vw] text-center">P2</th>
			<th class="w-[4vw] text-center">P3</th>
			<th class="w-[5vw] text-center">TOTAL</th>
			<th class="w-[6vw] text-center">INUSE</th>
			<th class="w-[6vw] text-center text-slate-400">DEPRECATED</th>
			<th class="w-[6vw] text-center">Test Cases</th>
		</tr>
		{#each reportData as data, index}
			<tr
				id={index.toString()}
				class="{highlightedRowIndex == index
					? 'bg-blue-400 text-white'
					: 'bg-white'} hover:bg-blue-300 hover:text-white"
				on:click={() => highlightRow(index, data.srv_name, data.id)}
			>
				<td class="border text-left pl-[7.5%]">{data.srv_name}</td>
				<td class="border text-center">{data.p1}</td>
				<td class="border text-center">{data.p2}</td>
				<td class="border text-center">{data.p3}</td>
				<td class="border text-center">{data.total}</td>
				<td class="border text-center">{data.inuse}</td>
				<td class="border text-center text-slate-500">{data.deprecated}</td>
				<td class="border text-center">{data.tested}</td>
			</tr>
			<tr id="nested-row-{index}" class="hidden w-[100vw]">
				{#if data.nested_amtrecs_summary.length != 0}
					<td colspan="8" class=" bg-white p-0">
						<div class="flex p-1 max-h-[45vh] flex-wrap overflow-y-scroll bg-white">
							{#each data.nested_amtrecs_summary as nrd}
								<div
									class="w-[11.5vw] h-[10vh] bg-slate-50 border m-2 flex flex-row justify-start p-1 hover:bg-green-100"
								>
									<a
										href="/test"
										class="w-full"
										id={nrd.id.toString()}
										on:click={() => setApiId(nrd.id)}
									>
										<div class="w-[100%]">
											<div class=" text-xs text-blue-700">
												{nrd.api_name}
											</div>
											<hr />
											<div class="flex flex-row mt-2 justify-evenly items-center">
												<div
													class="border rounded-md p-2 w-10 h-10 text-center text-blue-600 align-middle relative group"
												>
													{nrd.count}
													<span
														class="absolute top-0 text-xs text-left text-white p-2 bg-black rounded opacity-0 group-hover:opacity-100 -mt-8 ml-8 hidden group-hover:block"
													>
														total
													</span>
												</div>
												<div
													class="border rounded-md p-2 w-10 h-10 text-center text-green-600 align-middle relative group"
												>
													{nrd.passed}
													<span
														class="absolute top-0 text-xs text-left text-white p-2 bg-black rounded opacity-0 group-hover:opacity-100 -mt-8 ml-8 hidden group-hover:block"
													>
														passed
													</span>
												</div>
												<div
													class="border rounded-md p-2 w-10 h-10 text-center align-middle relative group"
												>
													{nrd.failed}
													<span
														class="absolute top-0 text-xs text-left text-white p-2 bg-black rounded opacity-0 group-hover:opacity-100 -mt-8 ml-8 hidden group-hover:block"
													>
														failed
													</span>
												</div>
											</div>
										</div>
									</a>
								</div>
							{/each}
						</div>
					</td>
				{:else}
					<td colspan="8" class=" bg-white p-2 text-center"> there is no inused api found </td>
				{/if}
			</tr>
		{/each}
		<tfoot>
			<tr class="border border-collapse bg-white">
				<th class="border text-center" colspan="7"
					>Total Test Cases
					<input
						type="button"
						value="Download"
						class="border bg-blue-500 pl-0.5 pr-0.5 text-white float-right rounded-md mr-2"
						on:click={download}
					/></th
				>
				<td class="border text-center text-red-600">{totalTestCount}</td>
			</tr>
		</tfoot>
	</table>
</div>

<!--<div>-->
<!--	<table class="w-full border border-separate break-all bg-white text-xs">-->
<!--		<tr class="bg-white text-left sticky top-10">-->
<!--			<th class="w-[9vw] text-center">Test Case ID</th>-->
<!--			<th class="w-[9vw] text-center">Module</th>-->
<!--			<th class="w-[9vw] text-center">Function Name</th>-->
<!--			<th class="w-[9vw] text-center">Pre Conditions</th>-->
<!--			<th class="w-[9vw] text-center">Input Test Data</th>-->
<!--			<th class="w-[9vw] text-center">CallbackMsg1</th>-->
<!--			<th class="w-[9vw] text-center">CallbackMsg2</th>-->
<!--			<th class="w-[9vw] text-center">Expected Result</th>-->
<!--			<th class="w-[9vw] text-center">Test Result</th>-->
<!--			<th class="w-[9vw] text-center">Comment</th>-->
<!--			<th class="w-[9vw] text-center">Tested By</th>-->
<!--			<th class="w-[9vw] text-center">Tested Date</th>-->
<!--		</tr>-->
<!--		{#each reportData as data}-->
<!--			{#each data.test_cases_summary as test_case}-->
<!--				<tr>-->
<!--					<td-->
<!--						class="border text-center"-->
<!--						title="TEST_CASE_{data.srv_name}_{test_case.api_name}_#{test_case.id}"-->
<!--						>{`TEST_CASE_..._#${test_case.id}`}</td-->
<!--					>-->
<!--					<td class="border text-center">{data.srv_name}</td>-->
<!--					<td class="border text-left">{test_case.api_name}</td>-->
<!--					<td class="border text-left">{test_case.api_content}</td>-->
<!--					<td class="border text-left" title={test_case.data_input}-->
<!--						>{test_case.data_input.slice(0, 18)}...</td-->
<!--					>-->
<!--					<td class="border text-left" title={test_case.callback_msg1}-->
<!--						>{test_case.callback_msg1.slice(0, 18)}...</td-->
<!--					>-->
<!--					<td class="border text-left" title={test_case.callback_msg2}-->
<!--						>{test_case.callback_msg2.slice(0, 20)}...</td-->
<!--					>-->
<!--					<td class="border text-center">PASS</td>-->
<!--					<td-->
<!--						class="border text-center {test_case.result == 'PASS'-->
<!--							? 'text-green-500'-->
<!--							: 'text-red-600'}">{test_case.result}</td-->
<!--					>-->
<!--					<td class="border text-left text-slate-500">{test_case.comment}</td>-->
<!--					<td class="border text-center">{test_case.tested_by}</td>-->
<!--					<td class="border text-center">{test_case.date_time}</td>-->
<!--				</tr>-->
<!--			{/each}-->
<!--		{/each}-->
<!--	</table>-->
<!--</div>-->
