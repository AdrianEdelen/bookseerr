export async function checkHealth(): Promise<string> {
	try {
		const res = await fetch(`${import.meta.env.VITE_API_URL}/health`);
		if (!res.ok) throw new Error('Bad response');
		return await res.text();
	} catch (e) {
		return `Error ${e}`;
	}
}
