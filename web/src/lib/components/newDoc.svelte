<script lang="ts">
	import { addApiMessageDocument } from '$lib/api/apimessage';
	import { fetchApiMessageDocumentList, what_you_focus_on } from '$lib/stores';

	let content: string = '';
	let title: string = '';
	let doc_type: string = 'c++';

	const saveScriptContent = async () => {
		if (title == '') {
			alert('Please enter title, can not save empty data');
			return;
		}
		if (content == '') {
			alert('Please enter some content, can not save empty data');
			return;
		}
		const data = {
			api_id: $what_you_focus_on!.api.id,
			title: title,
			doc_type: doc_type,
			content: content
		};

		const res = await addApiMessageDocument(data);
		console.log(res);
		// clear input
		title = '';
		doc_type = 'c++';
		content = '';
		await fetchApiMessageDocumentList(data.api_id);
	};
</script>

<div class="p-1.5 w-[50%] bg-white rounded-xl shadow-md flex-row text-sm">
	<div class="border w-full">
		<div class="flex flex-row justify-start w-full">
			Type:
			<label for="dt-1" class="cursor-pointer ml-5"
				><input
					type="radio"
					id="dt-1"
					name="doc_type"
					checked={doc_type == 'c++'}
					on:click={() => (doc_type = 'c++')}
				/> C++</label
			>
			<label for="dt-2" class="cursor-pointer ml-5"
				><input type="radio" id="dt-2" name="doc_type" on:click={() => (doc_type = 'sql')} /> SQL</label
			>
			<label for="dt-3" class="cursor-pointer ml-5"
				><input type="radio" id="dt-3" name="doc_type" on:click={() => (doc_type = 'txt')} /> Comment</label
			>
			<label for="dt-4" class="cursor-pointer ml-5"
				><input type="radio" id="dt-4" name="doc_type" on:click={() => (doc_type = 'v')} /> Checkpoint*</label
			>
			<label for="dt-5" class="cursor-pointer ml-5"
				><input type="radio" id="dt-5" name="doc_type" on:click={() => (doc_type = 'js')} />
				Server Logs</label
			>
		</div>
	</div>
	<div class="border w-full">
		<label for="title"
			>Title:<span class="text-gray-300">({$what_you_focus_on.api.api_name})</span>
			<input
				type="text"
				bind:value={title}
				class="border w-full p-0.5"
				placeholder="input a title"
			/></label
		>
	</div>
	<textarea
		name="script_content"
		id="script_content"
		bind:value={content}
		cols="30"
		rows="10"
		class="border w-full p-0.5 resize-none"
		placeholder=">>>"
	/>
	<input
		type="button"
		value="Save"
		class="border pl-2 pr-2 mt-2 bg-blue-400 text-white hover:bg-blue-600"
		on:click={saveScriptContent}
	/>
</div>
