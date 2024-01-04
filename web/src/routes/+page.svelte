<script lang="ts">
	import type { ApiServer, ApiMessage } from '$lib';
	import {
		addNewApiMessage,
		delApiMessageById,
		doApiMessageValidation,
		getApiMessageBySrvId,
		updateApiMessage
	} from '$lib/api/apimessage';
	import { addNewApiServer, getApiServerList } from '$lib/api/apiserver';
	import { onMount } from 'svelte';

	let curr_srv_api: ApiMessage[] = [];
	let srv_list: ApiServer[] = [];
	let select_srv_id: number = 1;

	let default_hdr: string = '';
	let selectedRowIndex: number;

	const highlightRow = (rowIndex: number) => {
		selectedRowIndex = rowIndex;
	};

	const submit_api = () => {
		const selected_srv = document.getElementById('srv_list') as HTMLSelectElement;
		let api_content = document.getElementById('api_content') as HTMLSelectElement;
		console.log(selected_srv.value, api_content.value);
		save_api(Number(selected_srv.value), api_content.value.toUpperCase());
		api_content.value = '';
	};

	const submit_srv = () => {
		let srv_name = document.getElementById('srv_name') as HTMLSelectElement;

		console.log(srv_name.value);
		save_srv(srv_name.value, default_hdr);
		srv_name.value = '';
		default_hdr = '';
	};
	const delApi = (evt: any) => {
		console.log(evt.target.dataset);
		if (confirm('Are you sure you want to delete this API?')) {
			del_api(Number(evt.target.dataset.api_id));
		}
	};

	const del_api = async (id: number) => {
		const res = await delApiMessageById(id);
		console.log(res);
		srv_list = await getApiServerList();
		curr_srv_api = await getApiMessageBySrvId(select_srv_id);
	};

	const apiValidation = (evt: any) => {
		validation_api(Number(evt.target.dataset.api_id), evt.target.dataset.validation);
	};

	const validation_api = async (id: number, curr_validation: string) => {
		const res = await doApiMessageValidation(id, curr_validation);
		console.log(res);

		srv_list = await getApiServerList();
		curr_srv_api = await getApiMessageBySrvId(select_srv_id);
	};

	const save_api = async (srv_id: number, api_content: string) => {
		if (api_content == '' || api_content == null) {
			alert('Please enter API content');
			return;
		}

		const res = await addNewApiMessage(srv_id, api_content);
		alert(res);
		srv_list = await getApiServerList();
		curr_srv_api = await getApiMessageBySrvId(select_srv_id);
	};

	const save_srv = async (srv_name: string, default_hdr: string) => {
		if (srv_name == '' || srv_name == null) {
			alert('Please enter Srv Name');
			return;
		}

		const res = await addNewApiServer(srv_name, default_hdr);
		alert(res);
		srv_list = await getApiServerList();
		curr_srv_api = await getApiMessageBySrvId(select_srv_id);
	};

	const handleSrvSelect = async (evt: any) => {
		select_srv_id = evt.currentTarget.value;
		curr_srv_api = await getApiMessageBySrvId(select_srv_id);
		console.log(`api_data get by ${select_srv_id}`);
		console.log(curr_srv_api);
	};

	const updateApiPriority = async (target: ApiMessage, action: string) => {
		if (action == 'downgrade') {
			let priority = target.priority > 1 ? target.priority - 1 : target.priority;
			let api: ApiMessage = {
				...target,
				priority: priority
			};
			console.log('downgrade:', api.priority, target.priority);
			const res = await updateApiMessage(api);
			console.log(res);
		} else {
			let priority = target.priority < 3 ? target.priority + 1 : target.priority;
			let api: ApiMessage = {
				...target,
				priority: priority
			};
			console.log('upgrade:', api.priority, target.priority);
			const res = await updateApiMessage(api);
			console.log(res);
		}
		curr_srv_api = await getApiMessageBySrvId(select_srv_id);
	};
	onMount(async () => {
		srv_list = await getApiServerList();
		curr_srv_api = await getApiMessageBySrvId(srv_list[0].id);
	});
</script>

<div class="w-full h-screen min-h-[75vh] p-1 bg-gray-200 text-xs">
	<div class="border bg-white w-full h-60 flex flex-row justify-between pt-1 pl-1">
		<div class="border w-[79vw] h-57">
			<div>
				<div>
					SRV NAME:

					<select name="srv_list" id="srv_list" class="ml-5 mr-5" on:change={handleSrvSelect}>
						{#each srv_list as srv}
							<option value={srv.id}>{srv.srv_name}</option>
						{/each}
					</select>
					<input
						type="submit"
						value="NEW API"
						class="border pr-4 pl-4 bg-red-600 text-white float-right rounded-sm"
						on:click={submit_api}
					/>
				</div>
				<hr />
				<div>
					<div>API CONTENT:</div>
					<div>
						<input
							class="border w-full"
							id="api_content"
							name="api_content"
							placeholder="Enter the Api Content."
						/>
					</div>
				</div>
			</div>
		</div>
		<div class="border w-[29vw] h-57">
			<div>
				SRV NAME:
				<input
					type="submit"
					value="NEW SRV"
					class="border pr-4 pl-4 bg-orange-600 text-white float-right rounded-sm"
					on:click={submit_srv}
				/>
				<input
					class="border w-full"
					id="srv_name"
					name="srv_name"
					placeholder="Enter the Srv Name."
				/>
				<input
					class="border w-full"
					id="default_hdr"
					name="default_hdr"
					bind:value={default_hdr}
					placeholder="Enter the default_hdr for the new Srv."
				/>
			</div>
		</div>
	</div>
	<div class="border bg-white w-full h-[74vh]">
		<div class="h-[44vh] overflow-y-scroll border">
			<table class="w-full border border-separate">
				<tr class="bg-white text-left sticky top-0">
					<th class="w-[2vw] text-center bg-blue-600 text-white">{curr_srv_api.length}</th>
					<th class="w-[9vw]">API_NAME</th>
					<th>API CONTENT</th>
					<th class="w-[4vw] text-center">VALID</th>
					<th class="w-[4vw] text-center">PRIORITY</th>
					<th class="w-[5vw] text-center">TEST CNT</th>
					<th class="w-[6vw] text-center">OPERATION</th>
				</tr>
				{#each curr_srv_api as api, index}
					<tr
						id={index.toString()}
						class="{selectedRowIndex == index
							? 'bg-blue-400 text-white'
							: ''} hover:bg-blue-300 hover:text-white"
						on:click={() => highlightRow(index)}
					>
						<td class="border text-center">{index + 1}</td>
						<td class="border">
							{api.api_name}
						</td>
						<td class="border">{api.api_content}</td>
						<td
							class="border text-center {api.is_inuse == 'Y'
								? `bg-green-400 text-white`
								: `bg-red-400 text-white`} rounded-sm">{api.is_inuse}</td
						>
						<td class="border text-center">
							<div>
								<input
									type="button"
									value="-"
									class=" bg-gray-300 pl-1 pr-1 hover:cursor-pointer rounded-sm"
									on:click={() => updateApiPriority(api, 'downgrade')}
								/>
								<span
									class="{api.priority == 1
										? `bg-red-400`
										: api.priority == 2
										? `bg-blue-400`
										: `bg-yellow-400`} text-white pl-1 pr-1">{api.priority}</span
								>
								<input
									type="button"
									value="+"
									class="bg-gray-300 pl-1 pr-1 hover:cursor-pointer rounded-sm"
									on:click={() => updateApiPriority(api, 'upgrade')}
								/>
							</div>
						</td>
						<td class="border text-center">{api.test_count}</td>
						<td class="border text-center float-left">
							<input
								type="button"
								value="DEL"
								class="text-red-500"
								data-api_id={api.id}
								on:click={delApi}
							/>
							<input
								type="button"
								value={api.is_inuse == 'Y' ? `INVLALID` : `VALID`}
								class="text-blue-500"
								data-api_id={api.id}
								data-validation={api.is_inuse}
								on:click={apiValidation}
							/>
						</td>
					</tr>
				{/each}
			</table>
		</div>
	</div>
</div>
