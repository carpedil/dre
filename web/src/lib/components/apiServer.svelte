<script lang="ts">
	import { onMount } from 'svelte';
	import ApiMessageWarpper from './api_message/apiMessageWarpper.svelte';
	import type { ApiMessage, ApiServer } from '$lib';
	import { what_you_focus_on, update_what_you_focus_on } from '$lib/stores';
	import { getApiServerList } from '$lib/api/apiserver';
	import { getApiMessageBySrvId } from '$lib/api/apimessage';

	let srv_list: ApiServer[] = [];
	let select_srv_id: number = 0;
	let select_srv_api: ApiMessage[] = [];
	onMount(async () => {
		srv_list = await getApiServerList();
		if ($what_you_focus_on.srv_id == -1) {
			await update_what_you_focus_on(srv_list[0].id, srv_list[0].api_list[0].id);
		}
	});

	const handleClick = async (evt: any) => {
		select_srv_id = evt.target.dataset.srv_id;
		select_srv_api = await getApiMessageBySrvId(select_srv_id);
		await update_what_you_focus_on(select_srv_id, select_srv_api[0].id);
	};
</script>

<div class="border w-[10vw] p-1">
	<span class="border-t-0 border-l-0 border-r-0 border">SRV LIST:</span>
	<div class="flex flex-col">
		{#each srv_list as srv}
			<label for={srv.id.toString()}>
				<input
					type="radio"
					name="srvlist"
					id={srv.id.toString()}
					value={srv.srv_name}
					checked={$what_you_focus_on != undefined
						? srv.id == $what_you_focus_on.srv_id
						: srv.id == srv_list[0].id}
					data-srv_id={srv.id}
					on:click={handleClick}
					class="border text-left w-auto p-0.5 hover:text-white hover:bg-red-500 focus:bg-red-600 focus:text-white rounded-sm"
				/>
				{srv.srv_name}
				<span
					class="w-[1.5vw] text-blue-400 text-xs border bg-white float-right text-center rounded-full"
					>{srv.api_list.length}</span
				>
			</label>
		{/each}
	</div>
</div>
<div class="w-[90vw] ml-1">
	<ApiMessageWarpper />
</div>
