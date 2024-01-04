<script lang="ts">
	import {
		what_you_focus_on,
		apiMessageDocumentList,
		deleteCurrentDoc,
		tabSet,
		renameCurrentDoc
	} from '$lib/stores';
	import { CodeBlock, Tab, TabGroup } from '@skeletonlabs/skeleton';
</script>

<div class="w-full h-[90vh] overflow-y-scroll bg-white p-2 text-black">
	<TabGroup justify="justify-start" class="text-gray-800 text-xs bg-white">
		{#if $apiMessageDocumentList.length > 0}
			{#each $apiMessageDocumentList as doc, index}
				<Tab bind:group={$tabSet} name="tab{index}" value={index}
					><input
						type="button"
						value={doc.title}
						on:click={() => tabSet.set(index)}
						on:dblclick={() => renameCurrentDoc($what_you_focus_on.api.id, doc.id, doc.title)}
					/></Tab
				>
			{/each}
		{:else}
			<Tab bind:group={$tabSet} name="tab0" value={0}
				><input type="button" value="Not Found" on:click={() => tabSet.set(0)} /></Tab
			>
		{/if}
		<svelte:fragment slot="panel">
			{#if $apiMessageDocumentList.length > 0}
				{#each $apiMessageDocumentList as doc, index}
					{#if $tabSet === index}
						<p class="pl-1 bg-slate-700 text-white rounded-none">
							<span class="text-gray-400 mr-1">#{doc.id}</span>
							{doc.created_at}
							<input
								type="button"
								value="X"
								class="p-0.5 cursor-pointer text-red-500"
								on:click={() => deleteCurrentDoc($what_you_focus_on.api.id, doc.id)}
							/>
						</p>
						<CodeBlock
							language={doc.doc_type}
							lineNumbers
							background="bg-slate-600"
							rounded="rounded-none"
							code={`${doc.content}`}
						/>
					{/if}
				{/each}
			{:else if $tabSet === 0}
				<p class="pl-1 bg-slate-400 text-white rounded-none">there is no documents available</p>
			{/if}
		</svelte:fragment>
	</TabGroup>
</div>
