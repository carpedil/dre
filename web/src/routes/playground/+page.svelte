<script lang="ts">
	import {
		executeApiMessageQueryScript,
		getApiMessageScriptList,
		saveApiMessageScriptList
	} from '$lib/api/apimessage';
	import { onMount } from 'svelte';
	import type { ApiMessageScript, QueryData } from '$lib';
	import { ExportToExcel } from '$lib/ExportToExcel';

	// 简易防抖
	let timeout = 0;
	let script_content = '';
	let output: QueryData = {
		headers: [],
		rows: []
	};
	let script_list: ApiMessageScript[] = [];

	onMount(async () => {
		// document = window.document as HTMLDocument;
		console.log(window);
		window.addEventListener('keydown', function (e) {
			if (e.key === 'F2') {
				clearTimeout(timeout);
				timeout = setTimeout(async () => {
					console.log(script_content.includes('SELECT'));
					if (script_content.length == 0) {
						// output = 'can not run empty string';
					} else {
						if (script_content.toLowerCase().includes('select') !== true) {
							// output = 'Only support select query, Any update or delete operation are not allowed';
							return;
						}
						// F2 被按下了
						console.log('F2 pressed!\n', script_content);
						output = await executeApiMessageQueryScript(script_content);
						console.log('output:\n', output);
					}
				}, 200);
			}
			if (e.key === 'Escape') {
				clearTimeout(timeout);
				timeout = setTimeout(() => {
					script_content = '';
					output = {
						headers: [],
						rows: []
					};
					console.log('ESC pressed!', script_content);
				}, 200);
			}
		});

		script_list = await getApiMessageScriptList();
	});

	const handleClick = async () => {
		if (script_content.length == 0) {
			alert('script is empty!');
			return;
		}
		let description = prompt('give a description for this script');

		let script: { description: string; content: string } = {
			description: description ? description : `${script_content}`,
			content: script_content
		};
		await saveApiMessageScriptList(script);
		script_list = await getApiMessageScriptList();
	};

	const handleScriptSelect = (script: string) => {
		script_content = script;
	};

	const handleDownload = () => {
		ExportToExcel(output.rows, 'data');
	};
</script>

<div class="w-[99.6vw] h-[100vh] m-0 border bg-white">
	<div class="h-[3vh] items-center p-0.5 bg-gray-300">
		<code>F2</code>: run <code>Esc</code>: clear
		<input type="button" value="save" class="border rounded-md pl-1 pr-1" on:click={handleClick} />
		{#if output.rows.length != 0}
			<input
				type="button"
				value="download"
				class="border rounded-md float-right mr-1 pl-1 pr-1 text-blue-600"
				on:click={handleDownload}
			/>
		{/if}
	</div>
	<div class="w-auto max-h-[45vh] h-[35vh] border border-b-blue-600 flex flex-row items-stretch">
		<textarea
			class="w-[80vw] h-[34.6vh] p-0.5 border resize-none bg-slate-50"
			bind:value={script_content}
		/>
		<div class="border bg-slate-200 w-[20vw] overflow-y-scroll">
			{#each script_list as script}
				<table class="flex flex-row items-center border-b-blue-600 border hover:bg-blue-300">
					<tr on:click={() => handleScriptSelect(script.content)}>
						<td class="w-[10vw] text-gray-400">{script.created_at}</td>
						<td class="w-[10vw] text-gray-500">{script.description}</td>
					</tr>
				</table>
			{/each}
		</div>
	</div>
	<div class="w-auto h-[62vh] border overflow-y-scroll">
		<table class="border w-full p-0.5">
			<tr class="w-full">
				{#each output.headers as header}
					<th class="text-left border flex-1">{header}</th>
				{/each}
			</tr>
			<tbody>
				{#each output.rows as row}
					<tr class="w-full border">
						{#each output.headers as _, index}
							<td class="flex-1">{row[index]}</td>
						{/each}
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
</div>
