<script lang="ts">
	import { what_you_focus_on } from '$lib/stores';
</script>

<span class="m-1 p-1 rounded-md" title="test results">
	<span class="pl-1 pr-1 text-xs text-white rounded-full bg-green-400"
		>{$what_you_focus_on.api.test_res.passed}</span
	>
	<span class="pl-1 pr-1 text-xs text-white rounded-full bg-red-400"
		>{$what_you_focus_on.api.test_res.failed}</span
	>
	<span class="pl-1 pr-1 text-xs text-white rounded-full bg-blue-400"
		>{$what_you_focus_on.api.test_res.tested}</span
	>
</span>
