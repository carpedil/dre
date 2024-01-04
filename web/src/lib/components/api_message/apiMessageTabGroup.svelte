<script lang="ts">
	import {
		update_what_you_focus_on,
		updateApiMessageUrgentStateById,
		what_you_focus_on
	} from '$lib/stores';
	import { Tab, TabGroup } from '@skeletonlabs/skeleton';
	import ApiMessageDocument from './apiMessageDocument.svelte';
	import ApiMessageInfo from './apiMessageInfo.svelte';
	import ApiMessageTestRecords from './apiMessageTestRecords.svelte';
	import ApiMessageTest from './apiMessageTest.svelte';
	import ApiMessageTestResultCount from './apiMessageTestResultCount.svelte';

	let tabSet: number = 0;
	const handleUpdate = async () => {
		let new_state = {
			id: $what_you_focus_on.api.id,
			state: $what_you_focus_on.api.mark_flag == 'Y' ? 'N' : 'Y'
		};
		await updateApiMessageUrgentStateById(new_state);
		await update_what_you_focus_on($what_you_focus_on.srv_id, $what_you_focus_on.api.id);
	};
</script>

<div class="flex flex-col justify-start border w-full min-h-[76.5vh] overflow-y-scroll">
	<TabGroup justify="justify-start" class="bg-slate-100 text-gray-800">
		<Tab bind:group={tabSet} name="tab1" value={0}>
			<span class="flex items-center">
				{$what_you_focus_on.api.api_name}<ApiMessageTestResultCount />
				<input
					type="button"
					class="cursor-pointer {$what_you_focus_on.api.mark_flag === 'Y'
						? 'text-red-600 '
						: 'text-slate-600'} text-lg"
					value="♥"
					on:dblclick={handleUpdate}
				/>
			</span>
		</Tab>
		<Tab bind:group={tabSet} name="tab2" value={1}>documents</Tab>
		<Tab bind:group={tabSet} name="tab3" value={2}>Testing</Tab>
		<Tab bind:group={tabSet} name="tab4" value={3}>records</Tab>
		<!-- Tab Panels --->
		<svelte:fragment slot="panel">
			{#if tabSet === 0}
				<ApiMessageInfo />
			{:else if tabSet === 1}
				<ApiMessageDocument />
			{:else if tabSet === 2}
				<ApiMessageTest />
			{:else if tabSet === 3}
				<ApiMessageTestRecords />
			{/if}
		</svelte:fragment>
	</TabGroup>
</div>
