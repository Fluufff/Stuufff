export type AuthInfo = {
	kind: 'ServiceAccount' | 'Bearer' | 'Disabled';
	subject: string;
	access: 'ClusterWide' | { teams: string[] };

	exp: number;
	iat: number;
	sub: string;
	email: string;
	name: string;
	picture: string;
	given_name: string;
	family_name: string;
	level: 'NONE' | 'READER' | 'REQUESTER' | 'EDITOR';
};

export function canEdit() {
	const currentAuth = auth;
	return currentAuth?.level === 'EDITOR';
}

// export const currentAuth = $state({ auth: null as AuthInfo | null });

// export const renew = async () => {
// 	fetch(`/api/v1/auth/whoami`)
// 		.then((resp) => {
// 			if (resp.status != 401) {
// 				return resp.json();
// 			}
// 		})
// 		.then((resp: AuthInfo) => {
// 			currentAuth.auth = resp;
// 		});
// };

const getAuth = async () => {
	return fetch(`/api/v1/auth/whoami`)
		.then((resp) => {
			if (resp.status != 401) {
				return resp.json();
			}
		})
		.then((resp: AuthInfo) => {
			return resp;
		});
};

export let auth = $state<AuthInfo | {}>({} as AuthInfo | {});
export const loadAuth = async () => {
	const newAuth = await getAuth();
	for (const key in newAuth) {
		(auth as Record<string, unknown>)[key] = (newAuth as Record<string, unknown>)[key];
	}
};
