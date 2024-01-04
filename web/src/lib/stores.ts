import type { ApiMessageDocument, ApiMessageTestRecord, WhatYouFocusNow } from '$lib';
import { writable } from 'svelte/store';
import {
	delApiMessageDocument,
	getApiMessageBySrvId,
	getApiMessageDocumentList,
	getApiMessageTestRecordList,
	updateApiMessageDocumentTitle,
	updateApiMessageUrgentState
} from './api/apimessage';

export let socketUrl = writable('');
export let remoteTestServerList = writable<string[]>([]);
export let foldFlag = writable<boolean>(true);
export let currApiMessageTestRecord = writable<ApiMessageTestRecord | undefined>(undefined);
export let receivedCallbackMessageList = writable<string[]>([]);
export let loading = writable<boolean>(false);
export let apiMessageDocumentList = writable<ApiMessageDocument[]>([]);
export let apiMessageTestCmtList = writable<ApiMessageDocument[]>([]);
export let apiMessageTestCheckPointList = writable<ApiMessageDocument[]>([]);
export let apiMessageTestRecordList = writable<ApiMessageTestRecord[]>([]);
export let sendingMessage = writable<string>('');
export let tabSet = writable<number>(0);

const initWhatYouFocusNow = (): WhatYouFocusNow => {
	return {
		srv_id: -1,
		api: {
			id: 0,
			srv_id: 0,
			srv_name: '',
			api_name: '',
			typ: '',
			api_content: '',
			api_param: [],
			test_count: 0,
			is_inuse: '',
			mark_flag: '',
			priority: 0,
			comment: '',
			test_res: {
				tested: 0,
				passed: 0,
				failed: 0
			}
		},
		api_list: []
	};
};
export let what_you_focus_on = writable<WhatYouFocusNow>(initWhatYouFocusNow());

export const update_what_you_focus_on = async (srv_id: number, api_id: number) => {
	const api_list = await getApiMessageBySrvId(srv_id);
	let init_gf = initWhatYouFocusNow();
	init_gf.srv_id = srv_id;
	init_gf.api_list = api_list;
	init_gf.api = api_list.filter((item) => item.id === api_id)[0];

	what_you_focus_on.set(init_gf);
	await fetchApiMessageDocumentList(api_id);
	await fetchApiMessageTestRecordList(api_id);
};

export const fetchApiMessageDocumentList = async (api_id: number) => {
	const list = await getApiMessageDocumentList(api_id);
	const doc_list = list.filter(
		(item) => item.doc_type === 'sql' || item.doc_type === 'c++' || item.doc_type === 'js'
	);
	const cmt_list = list.filter((item) => item.doc_type === 'txt');
	const cp_list = list.filter((item) => item.doc_type === 'v');
	apiMessageDocumentList.set(doc_list);
	apiMessageTestCmtList.set(cmt_list);
	apiMessageTestCheckPointList.set(cp_list);
};

export const fetchApiMessageTestRecordList = async (api_id: number) => {
	const list = await getApiMessageTestRecordList(api_id);
	apiMessageTestRecordList.set(list);
};

export const deleteCurrentDoc = async (api_id: number, doc_id: number) => {
	const is_continue = confirm('Are you sure to delete this document?');
	if (is_continue) {
		const res = await delApiMessageDocument(doc_id);
		console.log(res);
		await fetchApiMessageDocumentList(api_id);
	}
};

export const renameCurrentDoc = async (api_id: number, doc_id: number, title: string) => {
	const new_title = prompt('Please enter a new title for this document:', title);
	if (new_title) {
		if (new_title === title) {
			alert('The title is the same as the original one, no need to change.');
			return;
		}
		const res = await updateApiMessageDocumentTitle(doc_id, new_title);
		console.log(res);
		await fetchApiMessageDocumentList(api_id);
	}
};

export const updateApiMessageUrgentStateById = async (newState: { id: number; state: string }) => {
	const res = await updateApiMessageUrgentState(newState);
	console.log(res);
};
