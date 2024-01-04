<script lang="ts">
	import { currApiMessageTestRecord, loading } from '$lib/stores';
	import { load } from 'cheerio';
	import { createTwoFilesPatch } from 'diff';
	import { html, parse } from 'diff2html';
	import type { OutputFormatType } from 'diff2html/lib/types';

	let keyword: string = '';
	let diffHTML: string;

	const getRemainingMessages = (origin: string, search: string): string => {
		const index = origin.toUpperCase().indexOf(search.toUpperCase());
		if (index != -1) {
			return origin.substring(index + search.length);
		}
		return origin;
	};

	const handleRenderDiff = (outputFormat: OutputFormatType | undefined) => {
		if ($currApiMessageTestRecord != undefined) {
			if (keyword != '') {
				let msg1 = getRemainingMessages($currApiMessageTestRecord.callback_msg1, keyword);
				let msg2 = getRemainingMessages($currApiMessageTestRecord.callback_msg2, keyword);
				console.log(msg1 == msg2);
				let context: number =
					msg1.split('\n').length > msg2.split('\n').length
						? msg1.split('\n').length
						: msg2.split('\n').length;
				randerDiff(msg1, msg2, outputFormat, context);
			} else {
				let context: number =
					$currApiMessageTestRecord.callback_msg1.split('\n').length >
					$currApiMessageTestRecord.callback_msg2.split('\n').length
						? $currApiMessageTestRecord.callback_msg1.split('\n').length
						: $currApiMessageTestRecord.callback_msg1.split('\n').length;
				randerDiff(
					$currApiMessageTestRecord.callback_msg1,
					$currApiMessageTestRecord.callback_msg2,
					outputFormat,
					context
				);
			}
		}
	};

	const randerDiff = (
		msg1: string,
		msg2: string,
		outputFormat: OutputFormatType | undefined,
		context: number
	) => {
		console.log('context:', context);
		let diffString = createTwoFilesPatch('srv1', 'srv2', msg1, msg2, '', '', { context: context });
		let parsedDiff = parse(diffString);
		diffHTML = html(parsedDiff, {
			outputFormat: outputFormat,
			matching: 'lines'
		});

		let Cheer = load(diffHTML);
		// remove d2h-file-list-wrapper
		Cheer('.d2h-file-list-wrapper').remove();
		// remove d2h-file-collapse
		// Cheer('.d2h-file-collapse').remove();
		//remove d2h-file-header
		Cheer('.d2h-file-header').remove();
		diffHTML = Cheer.html();
		if (diffHTML.includes('File without changes')) {
			diffHTML = `
			<div class="d2h-files-diff">
				<div class="d2h-file-side-diff">
					<div class="d2h-code-wrapper">
						<table class="d2h-diff-table">
							<tbody class="d2h-diff-tbody">
								<tr>
									<td class="d2h-code-side-linenumber d2h-info" />
									<td class="d2h-info">
										<div class="d2h-code-side-line">@@ matched @@</div>
									</td>
								</tr>
								<tr>
									<td class="d2h-code-side-linenumber d2h-del d2h-change"> 1 </td>
									<td class="d2h-del d2h-change">
										<div class="d2h-code-side-line">
											<span class="d2h-code-line-prefix">+</span>
											<span class="d2h-code-line-ctn"><ins>${msg1}</ins></span>
										</div>
									</td>
								</tr>
							</tbody>
						</table>
					</div>
				</div>
				<div class="d2h-file-side-diff">
					<div class="d2h-code-wrapper">
						<table class="d2h-diff-table">
							<tbody class="d2h-diff-tbody">
								<tr>
									<td class="d2h-code-side-linenumber d2h-info" />
									<td class="d2h-info">
										<div class="d2h-code-side-line">@@ matched @@</div>
									</td>
								</tr>
								<tr>
									<td class="d2h-code-side-linenumber d2h-del d2h-change"> 1 </td>
									<td class="d2h-del d2h-change">
										<div class="d2h-code-side-line">
											<span class="d2h-code-line-prefix">+</span>
											<span class="d2h-code-line-ctn"><ins>${msg1}</ins></span>
										</div>
									</td>
								</tr>
							</tbody>
						</table>
					</div>
				</div>
			</div>
			`;
		}
	};
	$: {
		if ($currApiMessageTestRecord != undefined) {
			if (keyword != '') {
				handleRenderDiff('line-by-line');
			} else {
				handleRenderDiff('line-by-line');
			}
		}
	}
</script>

<div class="bg-white">
	{#if $currApiMessageTestRecord != undefined && $loading == false}
		<div class="w-full p-0.5">
			<input
				type="text"
				name="keyword"
				bind:value={keyword}
				class="w-[80%] pl-1 text-gray-400 rounded-sm cursor-auto"
				placeholder="Searching by keyword ."
			/>
			<div class=" w-fit float-right">
				<div class="flex flex-row justify-between">
					<label for="d-2"
						><input
							id="d-2"
							type="radio"
							name="display"
							checked
							on:click={() => handleRenderDiff('line-by-line')}
						/> unified</label
					>
					<label for="d-1"
						><input
							id="d-1"
							type="radio"
							name="display"
							on:click={() => handleRenderDiff('side-by-side')}
						/> split</label
					>
				</div>
			</div>
		</div>
		<div class="bg-white min-h-[50vh]">{@html diffHTML}</div>
	{:else if $loading == true}
		<div class="w-full h-[65vh] flex flex-col justify-center items-center">
			<div class="loading" />
			<div>
				loading...
				<input
					type="button"
					class="text-red-500 cursor-pointer"
					value="x"
					on:click={() => loading.set(false)}
				/>
			</div>
		</div>
	{/if}
</div>

<style>
	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	.loading {
		display: inline-block;
		width: 25px;
		height: 25px;
		border: 2px solid #ccc;
		border-top-color: #0074d9;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}
</style>
