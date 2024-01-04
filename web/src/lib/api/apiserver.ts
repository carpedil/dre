import type { ApiServer } from '$lib';

export const addNewApiServer = async (
	srv_name: string,
	default_hdr: string
): Promise<string | void> => {
	const req = await fetch(`/api/asc/add`, {
		method: 'POST',
		body: JSON.stringify({
			srv_name: srv_name,
			default_hdr: default_hdr
		})
	});

	const res = await req.json();
	if (res.code != 200) {
		alert(res.message);
		return;
	}
	return res.message;
};

export const getApiServerList = async (): Promise<ApiServer[]> => {
	const req = await fetch(`/api/asc/list`, {
		method: 'GET'
	});

	const res = await req.json();
	if (res.code != 200) {
		alert(res.message);
		return [];
	}
	return res.data;
};
