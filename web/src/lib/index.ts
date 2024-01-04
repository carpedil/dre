// place files you want to import through the `$lib` alias in this folder.
type TestResult = {
	tested: number;
	passed: number;
	failed: number;
};

export type Param = {
	key: string;
	value: string;
	is_required: boolean;
};
export type ApiMessage = {
	id: number;
	srv_id: number;
	srv_name: string;
	api_name: string;
	typ: string;
	api_content: string;
	api_param: Param[];
	test_count: number;
	is_inuse: string;
	mark_flag: string;
	priority: number;
	test_res: TestResult;
	comment: string;
};
export type ApiServer = {
	id: number;
	srv_name: string;
	default_hdr: string;
	api_list: ApiMessage[];
};
export type ApiMessageTestRecord = {
	id: number;
	api_id: number;
	api_content: string;
	result: string;
	callback_msg1: string;
	callback_msg2: string;
	comment: string;
	created_at: string;
};

export type Result<T> = {
	code: number;
	message: string;
	data: T;
};

export type ApiMessageDocument = {
	id: number;
	api_id: number;
	title: string;
	doc_type: string;
	content: string;
	created_at: string;
};

export type ApiMessageTestRecordsSummary = {
	id: number;
	srv_name: string;
	p1: number;
	p2: number;
	p3: number;
	total: number;
	inuse: number;
	deprecated: number;
	tested: number;
	nested_amtrecs_summary: NestedApiMessageTestRecordsDetail[];
	test_cases_summary: ApiMessageTestCase[];
};

export type NestedApiMessageTestRecordsDetail = {
	id: number;
	api_name: string;
	count: number;
	passed: number;
	failed: number;
};

export type ApiMessageTestCase = {
	id: number;
	api_name: string;
	api_content: string;
	data_input: string;
	callback_msg1: string;
	callback_msg2: string;
	result: string;
	comment: string;
	tested_by: string;
	date_time: string;
};

export type WhatYouFocusNow = {
	srv_id: number;
	api: ApiMessage;
	api_list: ApiMessage[];
};

export type ApiMessageStatisticsCounts = {
	total: number;
	p1: number;
	p2: number;
	p3: number;
	done: number;
	pending: number;
};

export type QueryData = {
	headers: string[];
	rows: string[][];
};

export type ApiMessageScript = {
	id: number;
	description: string;
	content: string;
	created_at: string;
};
