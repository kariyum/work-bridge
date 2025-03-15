```typescript
const discussionIdMatcher = /messages\/(?<id>\d)/;
const selectedDiscussion: string | undefined = $derived(
	discussionIdMatcher.exec(page.url.pathname)?.groups?.id || undefined
);
```