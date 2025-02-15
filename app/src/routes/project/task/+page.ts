export async function load({ parent, url }) {
    const index = url.searchParams.get('s')
    const result = { selectedIndex: index ? parseInt(index) : undefined };
    // const parentData = await parent();
    // parentData.tasksGlobalState.selectTask(result.selectedIndex ?? -1);
    return result;
}