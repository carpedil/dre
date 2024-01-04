<script lang="ts">
	import type { ApiMessageTestRecord } from '$lib';
	import {
		addApiMessageTestRecord,
		getApiMessageById,
		updateApiMessageHeaderWithHostName
	} from '$lib/api/apimessage';
	import {
		currApiMessageTestRecord,
		fetchApiMessageTestRecordList,
		foldFlag,
		loading,
		receivedCallbackMessageList,
		remoteTestServerList,
		sendingMessage,
		socketUrl,
		what_you_focus_on
	} from '$lib/stores';
	import Diffresult from '../diffresult.svelte';

	let socket: WebSocket;
	let srv_ip_list: string[] = [];
	let testHistData: ApiMessageTestRecord | undefined;
	let result: string = '';
	let comment: string = '';
	// 控制testHistData渲染
	let total_message_count = 0;
	let counter = 0;
	let receive_flag : boolean = false;

	const saveApiMessageTestRecord = async () => {
		if (result === '') {
			alert('Testing Faill or Pass ?');
			return;
		}
		currApiMessageTestRecord.update((c) => ({ ...c!, comment, result }));

		const res = await addApiMessageTestRecord($currApiMessageTestRecord!);
		currApiMessageTestRecord.update((c) => (c = undefined));
		alert(res);
		// reset comment & result initial state
		await reset_me();
	};
	const reset_me = async () => {
		result = '';
		comment = '';
		await fetchApiMessageTestRecordList($what_you_focus_on.api.id);
		$what_you_focus_on.api = await getApiMessageById($what_you_focus_on.api.id);
		const radios = document.getElementsByName('result');
		for (let i = 0; i < radios.length; i++) {
			const radio = radios[i] as HTMLInputElement;
			radio.checked = false;
		}
	};

	const handleMsgChange = async () => {
		console.log('handleMsgChange');
		console.log($sendingMessage);
		const new_msg = await updateApiMessageHeaderWithHostName($sendingMessage);
		sendingMessage.set(new_msg);
		currApiMessageTestRecord.set(undefined);
		receivedCallbackMessageList.set([]);
	};

	const handleSend = async () => {
		loading.set(true);
		receivedCallbackMessageList.set([]);

		srv_ip_list = getSrvIpList();
		total_message_count = srv_ip_list.length
		console.log("max message count : ",total_message_count,"current callback message list length:",$receivedCallbackMessageList.length)

		if ($sendingMessage.length <= 1) {
			alert(
				'Api message can not be send with nothing, Select the api to be tested and enter the correct test data.'
			);
			loading.set(false);
			return;
		}
		if (srv_ip_list.length == 0) {
			alert(
				'The IP address of the server to be tested cannot be empty. Select at least one of the server IP addresses to send test information.'
			);
			loading.set(false);
			return;
		}

		initEventHandle(srv_ip_list, $sendingMessage);
	};

	const getSrvIpList: () => string[] = () => {
		let ip_list: string[] = [];
		// Get all the checkboxes with the name "test_srv_list"
		let checkboxes = document.getElementsByName('test_srv_list');

		// Loop through each checkbox
		for (let i = 0; i < checkboxes.length; i++) {
			// Type cast the element to HTMLInputElement
			let checkbox = checkboxes[i] as HTMLInputElement;

			// Check if the checkbox is checked
			if (checkbox.checked) {
				// Add the value of the checked checkbox to the selectedValues array
				ip_list.push(checkbox.value);
			}
		}

		// Now you have an array of the selected checkbox values
		console.log(ip_list);
		return ip_list;
	};

	const initEventHandle = (ip_list: string[], msg: string) => {
		socket = new WebSocket(`${$socketUrl}`);
		counter = 0;
		socket.onmessage = function (event) {
			console.log("message counter: ",counter);
			$receivedCallbackMessageList.push(event.data);
			counter++;
			console.log('new message>>:');
			console.log(JSON.stringify(event.data, null, 4));
			if (counter == total_message_count) {
				// this means all message has been received
				receive_flag = true
			}
		};

		socket.onopen = function (e) {
			console.log(`[open] Connection established @ ${$socketUrl}`);
			console.log(`Sending to server | ${ip_list.toString()}>${msg}`);
			socket.send(`${ip_list.toString()}>${msg}`);
		};

		socket.onerror = function (error) {
			console.log(`[error]${JSON.stringify(error, null, 4)}`);
		};
	};

	$: {
		if (receive_flag) {
			console.log('all callback message received',$receivedCallbackMessageList.length,$receivedCallbackMessageList)
			testHistData = undefined;
			if (counter == 2) {
				testHistData = {
					id: 0,
					api_id: $what_you_focus_on.api.id,
					api_content: $sendingMessage,
					result: '',
					callback_msg1: $receivedCallbackMessageList.pop()!,
					callback_msg2: $receivedCallbackMessageList.pop()!,
					comment: '',
					created_at: ''
				};
			} else {
				testHistData = {
					id: 0,
					api_id: $what_you_focus_on.api.id,
					api_content: $sendingMessage,
					result: '',
					callback_msg1: $receivedCallbackMessageList.pop()!,
					callback_msg2: '',
					comment: '',
					created_at: ''
				};
			}
			console.log("receivedCallbackMessageList should be empty", $receivedCallbackMessageList.length == 0)
			currApiMessageTestRecord.set(testHistData);
			loading.set(false);
			// reset counter and flag
			counter = 0;
			receive_flag = false
		}
	}
</script>

<div class="flex flex-row w-full bg-white border">
	<div class="border {$foldFlag ? `hidden` : `w-[29vw]`} min-h-[76.5vh] p-1">
		{#each $what_you_focus_on.api.api_param as param}
			<div class="border flex flex-col">
				<label for={param.key} class="pl-1 border"
					>{param.key}
					<input
						type="text"
						name={param.key}
						id={param.key}
						bind:value={param.value}
						on:change={() => (param.value = param.value.toUpperCase())}
						required={param.is_required}
						class="w-[20vw] pl-1 float-right border text-left text-slate-600 bg-slate-300"
					/>
				</label>
			</div>
		{/each}
	</div>

	<div class={$foldFlag ? `w-full` : `w-[68vw]`}>
		<div
			class="w-full flex flex-row justify-between bg-white border-2 border-b-black h-fit sticky top-0"
		>
			<div class="flex flex-row">
				<input
					type="button"
					name="fold_btn"
					id="fold_btn01"
					value={$foldFlag ? 'Edit' : '<<<'}
					on:click={() => foldFlag.set(!$foldFlag)}
					class="pl-1.5 pr-1.5 mr-5 bg-blue-600 text-white rounded-sm cursor-pointer"
				/>
				{#each $remoteTestServerList as srv_addr, index}
					<label for={`id_${index}`} class="mr-4 cursor-pointer"
						><input
							type="checkbox"
							value={srv_addr}
							id={`id_${index}`}
							name="test_srv_list"
							checked
						/>
						{srv_addr.toUpperCase()}</label
					>
				{/each}
			</div>
			<input
				type="button"
				value="SEND"
				class="float-right border pl-1.5 pr-1.5 bg-blue-600 text-white rounded-sm cursor-pointer"
				on:click={handleSend}
			/>
		</div>
		<div class="border w-full overflow-y-scroll p-1">
			<textarea
				name="msg"
				id="msg"
				class="w-full resize-none"
				rows="3"
				bind:value={$sendingMessage}
				on:change={handleMsgChange}
			/>
		</div>
		<div class="w-full flex flex-row justify-between p-0.5 bg-gray-200">
			<div class="flex flex-row">
				<label for="r-1" class="cursor-pointer"
					><input type="radio" id="r-1" name="result" on:click={() => (result = 'PASS')} /> PASS</label
				>
				<label for="r-2" class="cursor-pointer"
					><input type="radio" id="r-2" name="result" on:click={() => (result = 'FAIL')} /> FAIL</label
				>
				<label for="cmt-1"
					><input
						type="text"
						id="cmt-1"
						name="cmt"
						bind:value={comment}
						placeholder="Comments for test result."
						class="ml-2 pl-1 w-[43vw] text-gray-400"
					/></label
				>
			</div>

			<div>
				<input
					type="button"
					name="save"
					value="SAVE"
					class="float-right bg-blue-600 pl-1.5 pr-1.5 text-white rounded-sm cursor-pointer"
					on:click={saveApiMessageTestRecord}
				/>
			</div>
		</div>
		<Diffresult />
	</div>
</div>
