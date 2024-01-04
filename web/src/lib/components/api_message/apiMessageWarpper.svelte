<script lang="ts">
	import {
		currApiMessageTestRecord,
		foldFlag,
		receivedCallbackMessageList,
		sendingMessage,
		update_what_you_focus_on,
		what_you_focus_on
	} from '$lib/stores';
	import { getApiMessageById } from '$lib/api/apimessage';
	import ApiMessageTabGroup from './apiMessageTabGroup.svelte';

	const handleApiClick = async (evt: any) => {
		receivedCallbackMessageList.set([]);
		currApiMessageTestRecord.set(undefined);
		foldFlag.set(true);

		let active_id = evt.target.id;
		let api = await getApiMessageById(active_id);
		await update_what_you_focus_on(api.srv_id, api.id);
	};

	let temp_msg: string = '';
	$: {
		let tmp_msg = '';
		$what_you_focus_on.api.api_param.forEach((p) => {
			tmp_msg += `${p.key}=${
				p.value.includes(' ') && p.key != 'HDR'
					? `"${p.value.toUpperCase()}"`
					: `${p.value.toUpperCase()}`
			} `;
		});
		temp_msg = `${$what_you_focus_on.api.api_name} ${tmp_msg}`.trim();
		sendingMessage.set(temp_msg);
	}
</script>

<!-- container -->
<div class="border bg-white h-[15vh] p-1">
	<span class="border border-t-0 border-l-0 border-r-0">API LIST:</span>
	<div class="h-[12vh] w-full float-left overflow-y-scroll scroll-auto">
		{#each $what_you_focus_on.api_list as api}
			<input
				type="button"
				id={api.id.toString()}
				value="{api.api_name}{api.mark_flag === 'Y' ? ' (♥)' : ''}"
				class="border {$what_you_focus_on.api.id == api.id && api.is_inuse == 'Y'
					? 'bg-blue-500'
					: api.is_inuse == 'Y' && api.priority == 1
					? `bg-slate-500`
					: api.is_inuse == 'Y' && api.priority == 2
					? `bg-slate-400`
					: `bg-red-400 `} p-1 m-1 text-yellow-50 hover:bg-blue-400 rounded-md"
				on:click={handleApiClick}
				disabled={api.is_inuse != 'Y'}
			/>
		{/each}
	</div>
</div>
<div class="w-full h-fit] bg-blue-400 text-white p-0.5 m-0.5">
	{$what_you_focus_on.api.api_content}
</div>
<div class="bg-white w-full flex flex-row justify-between">
	<ApiMessageTabGroup />
</div>
