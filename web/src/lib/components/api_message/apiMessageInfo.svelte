<script lang="ts">
	import {
		apiMessageTestCheckPointList,
		apiMessageTestCmtList,
		deleteCurrentDoc,
		what_you_focus_on
	} from '$lib/stores';
	import NewDoc from '../newDoc.svelte';
</script>

<div class="bg-white p-1.5">
	<div class="p-1.5 mx-auto bg-white rounded-xl shadow-md flex items-start space-x-4 text-sm">
		<NewDoc />
		<div class="w-[50%]">
			<div class="text-xl font-medium text-black">Basic Info</div>
			<table class="border w-full">
				<tr>
					<td class="w-[15%] border">ID</td>
					<td class="w-[85%] border">#{$what_you_focus_on.api.id}</td>
				</tr>
				<tr>
					<td class="border">SERVER NAME</td>
					<td class="border">{$what_you_focus_on.api.srv_name}</td>
				</tr>
				<tr>
					<td class="border">API NAME</td>
					<td class="border"> {$what_you_focus_on.api.api_name}</td>
				</tr>
				<tr>
					<td class="border">TYPE</td>
					<td class="border"> {$what_you_focus_on.api.typ}</td>
				</tr>
				<!-- <tr>
					<td class="border">CONTENT</td>
					<td class="border"> {$what_you_focus_on.api.api_content}</td>
				</tr> -->
				<tr>
					<td class="border">IS_INUSE</td>
					<td class="border"> {$what_you_focus_on.api.is_inuse}</td>
				</tr>
				<tr>
					<td class="border">COMMENT</td>
					<td class="border"> {$what_you_focus_on.api.comment}</td>
				</tr>
			</table>
		</div>
	</div>
	<div
		class="{$apiMessageTestCmtList.length > 0 || $apiMessageTestCheckPointList.length > 0
			? ''
			: 'hidden'} p-1.5 mx-auto bg-white rounded-md mt-2 shadow-md flex items-center space-x-4 text-sm text-gray-500"
	>
		<div class="w-[50%]">
			{#if $apiMessageTestCmtList.length > 0}
				<table class="w-full">
					<thead>
						<tr>
							<td />
							<td class="w-[2vw]">#</td>
							<td class="w-[9vw]">EventTime</td>
							<td class="border">Comments</td>
						</tr>
					</thead>
					<tbody>
						{#each $apiMessageTestCmtList as cmt}
							<tr class="border">
								<td class="text-center"
									><input
										type="button"
										value="X"
										class="text-red-500 cursor-pointer"
										on:click={() => deleteCurrentDoc($what_you_focus_on.api.id, cmt.id)}
									/></td
								>
								<td>{cmt.id}</td>
								<td>{cmt.created_at}</td>
								<td>{cmt.title} | {cmt.content}</td>
							</tr>
						{/each}
					</tbody>
				</table>
			{/if}
		</div>
		<div class="w-[50%]">
			{#if $apiMessageTestCheckPointList.length > 0}
				<table class="w-full">
					<thead>
						<tr>
							<td />
							<td class="w-[2vw]">#</td>
							<td class="w-[9vw]">EventTime</td>
							<td class="border">CheckPoints</td>
						</tr>
					</thead>
					<tbody>
						{#each $apiMessageTestCheckPointList as cmt}
							<tr class="border">
								<td class="text-center"
									><input
										type="button"
										value="X"
										class="text-red-500 cursor-pointer"
										on:click={() => deleteCurrentDoc($what_you_focus_on.api.id, cmt.id)}
									/></td
								>
								<td>{cmt.id}</td>
								<td>{cmt.created_at}</td>
								<td>{cmt.title} | {cmt.content}</td>
							</tr>
						{/each}
					</tbody>
				</table>
			{/if}
		</div>
	</div>
</div>
