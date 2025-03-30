export async function checkHealth(): Promise<string> {
	try {
		const apiUrl = import.meta.env.VITE_BACKEND_API_URL;
		const res = await fetch(`${apiUrl}/requests`);
		if (!res.ok) throw new Error('Bad response');
		return await res.text();
	} catch (e) {
		return `Error ${e}`;
	}
}
