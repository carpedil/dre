import type {
	ApiMessage,
	ApiMessageDocument,
	ApiMessageScript,
	ApiMessageStatisticsCounts,
	ApiMessageTestRecord,
	ApiMessageTestRecordsSummary,
	QueryData
} from '$lib';

export const doApiMessageValidation = async (
	id: number,
	curr_validation: string
): Promise<string> => {
	const req = await fetch(`/api/amc/validation`, {
		method: 'POST',
		body: JSON.stringify({
			id: id,
			validation: curr_validation == 'Y' ? 'N' : 'Y'
		})
	});

	const res = await req.json();
	if (res.code != 200) {
		alert(res.message);
		return '';
	}
	return res.message;
};

export const delApiMessageById = async (id: number): Promise<string> => {
	const req = await fetch(`/api/amc/del`, {
		method: 'POST',
		body: JSON.stringify({
			id: id
		})
	});

	const res = await req.json();
	if (res.code != 200) {
		alert(res.message);
		return '';
	}
	return res.message;
};

export const getApiMessageById = async (id: number): Promise<ApiMessage | any> => {
	const req = await fetch(`/api/amc/id/${id}`, {
		method: 'GET'
	});

	const res = await req.json();
	if (res.code != 200) {
		alert(res.message);
		return;
	}
	return res.data;
};

export const getApiMessageBySrvId = async (id: number): Promise<ApiMessage[]> => {
	const req = await fetch(`/api/amc/srv_id/${id}`, {
		method: 'GET'
	});

	const res = await req.json();
	if (res.code != 200) {
		alert(res.message);
		return [];
	}
	return res.data;
};

export const addNewApiMessage = async (srv_id: number, api_content: string): Promise<string> => {
	const req = await fetch(`/api/amc/add`, {
		method: 'POST',
		body: JSON.stringify({
			srv_id: srv_id,
			api_content: api_content
		})
	});

	const res = await req.json();
	if (res.code != 200) {
		alert(res.message);
		return '';
	}
	return res.message;
};

export const updateApiMessage = async (api: ApiMessage): Promise<string> => {
	const req = await fetch('/api/amc/update', {
		method: 'POST',
		body: JSON.stringify(api)
	});

	const res = await req.json();
	if (res.code != 200) {
		alert(res.message);
		return '';
	}
	return res.message;
};

export const addApiMessageTestRecord = async (record: ApiMessageTestRecord): Promise<string> => {
	const req = await fetch('/api/amr/record/save', {
		method: 'POST',
		body: JSON.stringify(record)
	});

	const res = await req.json();
	if (res.code != 200) {
		alert(res.message);
		return '';
	}
	return res.message;
};

export const getApiMessageTestRecordList = async (
	api_id: number
): Promise<ApiMessageTestRecord[]> => {
	const req = await fetch(`/api/amr/record/list/${api_id}`, {
		method: 'GET'
	});

	const res = await req.json();
	if (res.code != 200) {
		alert(res.message);
		return [];
	}
	return res.data;
};

export const updateApiMessageHeaderWithHostName = async (message: string): Promise<string> => {
	const req = await fetch(`/api/amc/hostname`, {
		method: 'POST',
		body: JSON.stringify({
			message: message
		})
	});

	const res = await req.json();
	if (res.code != 200) {
		alert(res.message);
		return '';
	}
	console.log(res.data);
	return res.data;
};

export const addApiMessageDocument = async (data: {
	api_id: number;
	title: string;
	doc_type: string;
	content: string;
}): Promise<string> => {
	const req = await fetch(`/api/amd/doc/add`, {
		method: 'POST',
		body: JSON.stringify({
			api_id: data.api_id,
			title: data.title,
			doc_type: data.doc_type,
			content: data.content
		})
	});

	const res = await req.json();
	if (res.code != 200) {
		alert(res.message);
		return '';
	}
	console.log(res);
	return res.message;
};

export const delApiMessageDocument = async (id: number): Promise<string> => {
	const req = await fetch(`/api/amd/doc/del/${id}`, {
		method: 'POST'
	});

	const res = await req.json();
	if (res.code != 200) {
		alert(res.message);
		return '';
	}
	console.log(res);
	return res.message;
};

export const updateApiMessageDocumentTitle = async (id: number, title: string): Promise<string> => {
	const req = await fetch(`/api/amd/doc/update/${id}`, {
		method: 'POST',
		body: JSON.stringify({
			new_title: title
		})
	});

	const res = await req.json();
	if (res.code != 200) {
		alert(res.message);
		return '';
	}
	console.log(res);
	return res.message;
};

export const getApiMessageDocumentList = async (api_id: number): Promise<ApiMessageDocument[]> => {
	const req = await fetch(`/api/amd/doc/list/${api_id}`, {
		method: 'GET'
	});

	const res = await req.json();
	if (res.code != 200) {
		alert(res.message);
		return [];
	}
	console.log(res);
	return res.data;
};

export const getApiMessageTestRecordsSummaryList = async (): Promise<
	ApiMessageTestRecordsSummary[]
> => {
	const req = await fetch(`/api/report/record/summary`, {
		method: 'GET'
	});

	const res = await req.json();
	if (res.code != 200) {
		alert(res.message);
		return [];
	}
	console.log(res);
	return res.data;
};

export const getApiMessageStatisticsCounts = async (): Promise<ApiMessageStatisticsCounts> => {
	const req = await fetch(`/api/report/record/statistic/count`, {
		method: 'GET'
	});

	const res = await req.json();
	return res.data;
};

export const updateApiMessageUrgentState = async (newState: {
	id: number;
	state: string;
}): Promise<string> => {
	const req = await fetch(`/api/amc/state/update`, {
		method: 'POST',
		body: JSON.stringify({
			id: newState.id,
			state: newState.state
		})
	});

	const res = await req.json();
	return res.data;
};

export const executeApiMessageQueryScript = async (script: string): Promise<QueryData> => {
	const req = await fetch(`/api/amc/script/execute`, {
		method: 'POST',
		body: JSON.stringify({
			script: script
		})
	});

	const res = await req.json();
	return res.data;
};

export const getApiMessageScriptList = async (): Promise<ApiMessageScript[]> => {
	const req = await fetch(`/api/ams/list`, {
		method: 'GET'
	});

	const res = await req.json();
	if (res.code != 200) {
		alert(res.message);
		return [];
	}
	return res.data;
};

export const saveApiMessageScriptList = async (data: {
	description: string;
	content: string;
}): Promise<string> => {
	const req = await fetch(`/api/ams/save`, {
		method: 'POST',
		body: JSON.stringify(data)
	});

	const res = await req.json();
	if (res.code != 200) {
		alert(res.message);
		return '';
	}
	return res.message;
};
