'use client';
import { useState } from 'react';
export default function ConsolePage() {
  const [apiName, setApiName] = useState('my-api');
  const [upstream, setUpstream] = useState('http://backend:3000');
  const [pathPrefix, setPathPrefix] = useState('/api/v1/my-api');
  const [result, setResult] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [tab, setTab] = useState<'create' | 'traffic' | 'ratelimit' | 'openapi'>('create');
  const apiUrl = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080';
  const send = async (path: string, body: object) => {
    setLoading(true);
    try {
      const r = await fetch(`${apiUrl}${path}`, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(body) });
      setResult(JSON.stringify(await r.json(), null, 2));
    } catch (e) { setResult(`Error: ${e}`); }
    finally { setLoading(false); }
  };
  return (
    <div className="p-6 space-y-6">
      <h1 className="text-2xl font-bold">Gateway Console</h1>
      <div className="flex gap-2">
        {(['create', 'traffic', 'ratelimit', 'openapi'] as const).map((t) => (
          <button key={t} onClick={() => setTab(t)} className={`px-4 py-2 rounded-md text-sm font-medium ${tab === t ? 'bg-primary text-primary-foreground' : 'bg-muted text-muted-foreground hover:bg-accent/50'}`}>{t === 'ratelimit' ? 'Rate Limit' : t === 'openapi' ? 'OpenAPI' : t.charAt(0).toUpperCase() + t.slice(1)}</button>
        ))}
      </div>
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="border border-border rounded-lg p-4 space-y-4">
          {tab === 'create' && (<>
            <h2 className="font-semibold">Register API</h2>
            <div><label className="text-sm font-medium">API Name</label><input value={apiName} onChange={(e) => setApiName(e.target.value)} className="mt-1 w-full px-3 py-2 border border-input rounded-md bg-background text-sm" /></div>
            <div><label className="text-sm font-medium">Upstream URL</label><input value={upstream} onChange={(e) => setUpstream(e.target.value)} className="mt-1 w-full px-3 py-2 border border-input rounded-md bg-background text-sm" /></div>
            <div><label className="text-sm font-medium">Path Prefix</label><input value={pathPrefix} onChange={(e) => setPathPrefix(e.target.value)} className="mt-1 w-full px-3 py-2 border border-input rounded-md bg-background text-sm" /></div>
            <button onClick={() => send('/api/v1/gateway/apis', { name: apiName, upstream_url: upstream, path_prefix: pathPrefix })} disabled={loading} className="px-4 py-2 bg-primary text-primary-foreground rounded-md text-sm font-medium hover:opacity-90 disabled:opacity-50">{loading ? 'Creating...' : 'Register API'}</button>
          </>)}
          {tab === 'traffic' && (<>
            <h2 className="font-semibold">Traffic Analysis</h2>
            <button onClick={() => send('/api/v1/gateway/traffic', { api_id: 'api-001', time_range_minutes: 60 })} disabled={loading} className="px-4 py-2 bg-primary text-primary-foreground rounded-md text-sm font-medium hover:opacity-90 disabled:opacity-50">{loading ? 'Analyzing...' : 'Analyze Traffic'}</button>
          </>)}
          {tab === 'ratelimit' && (<>
            <h2 className="font-semibold">Configure Rate Limit</h2>
            <button onClick={() => send('/api/v1/gateway/ratelimit', { api_id: 'api-001', strategy: 'token_bucket', requests_per_minute: 1000, burst_size: 100 })} disabled={loading} className="px-4 py-2 bg-primary text-primary-foreground rounded-md text-sm font-medium hover:opacity-90 disabled:opacity-50">{loading ? 'Configuring...' : 'Set Rate Limit'}</button>
          </>)}
          {tab === 'openapi' && (<>
            <h2 className="font-semibold">Generate OpenAPI Spec</h2>
            <button onClick={() => send('/api/v1/gateway/openapi', { api_id: 'api-001', format: 'json' })} disabled={loading} className="px-4 py-2 bg-primary text-primary-foreground rounded-md text-sm font-medium hover:opacity-90 disabled:opacity-50">{loading ? 'Generating...' : 'Generate Spec'}</button>
          </>)}
        </div>
        <div className="border border-border rounded-lg p-4">
          <h2 className="font-semibold mb-2">Response</h2>
          <pre className="bg-muted rounded-md p-4 text-xs font-mono overflow-auto max-h-96 whitespace-pre-wrap">{result || 'No response yet'}</pre>
        </div>
      </div>
    </div>
  );
}
