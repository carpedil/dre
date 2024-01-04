<script lang="ts">
	// Your selected Skeleton theme:
	import '@skeletonlabs/skeleton/themes/theme-skeleton.css';
	// This contains the bulk of Skeletons required styles
	import '@skeletonlabs/skeleton/styles/skeleton.css';
	// Finally, your application's global stylesheet (sometimes labeled 'app.css')
	import '../app.postcss';
	import hljs from 'highlight.js';
	import 'highlight.js/styles/github-dark.css';
	import { storeHighlightJs } from '@skeletonlabs/skeleton';
	import { onMount } from 'svelte';
	import { remoteTestServerList, socketUrl } from '$lib/stores';
	import { getAppConfig } from '$lib/api/config';
	import type { ApiMessageStatisticsCounts } from '$lib';
	import { getApiMessageStatisticsCounts } from '$lib/api/apimessage';

	storeHighlightJs.set(hljs);
	let scd: ApiMessageStatisticsCounts;

	onMount(async () => {
		const res = await getAppConfig();

		socketUrl.set(res.ws_url);
		remoteTestServerList.set(res.srv_addrs);
		scd = await getApiMessageStatisticsCounts();
	});
</script>

<nav
	class="flex flex-row list-none justify-start bg-slate-600 text-white text-xs p-2 sticky top-0 z-0"
>
	<ul class="flex flex-row list-none justify-start w-[100%]">
		<li class="p-2 ml-10"><a href="/">HOME</a></li>
		<li class="p-2 ml-10"><a href="/test.html">TEST</a></li>
		<li class="p-2 ml-10"><a href="/report.html">REPORT</a></li>
		<li class="p-2 ml-10"><a href="/playground.html">REPL</a></li>
	</ul>
	{#if scd != undefined}
		<span class="float-right flex w-auto rounded-md">
			<div title="total api counts" class="p-2 text-center cursor-help rounded-sm bg-slate-500">
				total({scd.total})
			</div>
			<div title="api with priority 1" class="p-2 text-center cursor-help rounded-sm bg-green-500">
				p1({scd.p1})
			</div>
			<div title="api with priority 2" class="p-2 text-center cursor-help rounded-sm bg-blue-500">
				p2({scd.p2})
			</div>
			<div title="api with priority 3" class="p-2 text-center cursor-help rounded-sm bg-orange-500">
				p3({scd.p3})
			</div>
			<div
				title="tested api count"
				class="p-2 text-center cursor-help rounded-sm bg-white text-black"
			>
				done({scd.done})
			</div>
			<div
				title="api count to be tested"
				class="p-2 text-center cursor-help rounded-sm bg-purple-500"
			>
				pending({scd.pending})
			</div>
		</span>
	{/if}
</nav>
<slot />
