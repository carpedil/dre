export const getAppConfig = async (): Promise<{ ws_url: string; srv_addrs: string[] }> => {
	const req = await fetch('/api/app/config', {
		method: 'GET'
	});
	return await req.json();
};
