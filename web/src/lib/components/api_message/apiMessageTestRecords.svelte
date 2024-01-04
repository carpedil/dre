<script lang="ts">
	import type { ApiMessageTestRecord } from '$lib';
	import { apiMessageTestRecordList, currApiMessageTestRecord } from '$lib/stores';
	import { onDestroy } from 'svelte';
	import Diffresult from '../diffresult.svelte';

	const showDiffResult = (data: ApiMessageTestRecord) => {
		currApiMessageTestRecord.set(data);
	};
	onDestroy(() => {
		currApiMessageTestRecord.set(undefined);
		console.log('currApiMessageTestRecord set undefined');
	});
</script>

<div class="w-full h-[90vh] bg-white text-gray-900 flex flex-row justify-between">
	<div class="w-[20%] h-[100%] pl-2 overflow-hidden border">
		{#each $apiMessageTestRecordList as data, index}
			<ul>
				<input
					type="button"
					title={data.comment}
					class="cursor-pointer text-sm text-left text-black bg-slate-50 p-0.5 w-full"
					value="{index + 1} - {data.result} | {data.created_at} | {data.comment}"
					on:click={() => showDiffResult(data)}
				/>
			</ul>
		{/each}
	</div>

	{#if $currApiMessageTestRecord != undefined}
		<div class="w-[79%] h-[100%] border p-1 overflow-y-scroll">
			<p class=" text-black bg-slate-50">
				<textarea
					class="w-full resize-none"
					rows="4"
					value={$currApiMessageTestRecord?.api_content}
				/>
			</p>
			<p class="text-red-500 w-full border bg-white">{$currApiMessageTestRecord?.comment}</p>
			<Diffresult />
		</div>
	{/if}
</div>
