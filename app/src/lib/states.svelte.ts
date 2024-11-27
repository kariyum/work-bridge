// used for storing added tasks that are not saved yet!
// way of communication between /project/task and /project
export const tasksStore = $state<{ tasks: Array<TaskClass>, selected: number }>({ tasks: [], selected: -1 });

export class TaskClass {
    title: string = $state('');
    assignee_id: string = $state('');
    status: string = $state('');
    content: string = $state('')
    deadline: string = $state('');
    budget?: number = $state(0);
    skills: string = $state('');

    constructor(
        title: string = '',
        assignee_id: string = 'None',
        status: string = 'TODO',
        content: string = '',
        deadline: string = '',
        budget: number | undefined = undefined,
        skills: string = ''
    ) {
        this.title = title;
        this.assignee_id = assignee_id;
        this.status = status;
        this.content = content;
        this.deadline = deadline;
        this.budget = budget;
        this.skills = skills;
    }


    static fromSelf(self: TaskClass): TaskClass {
        return new TaskClass(
            self.title,
            self.assignee_id,
            self.status,
            self.content,
            self.deadline,
            self.budget,
            self.skills
        )
    }

    // assign from 
    assignFrom(task: TaskClass): void {
        this.title = task.title;
        this.assignee_id = task.assignee_id;
        this.status = task.status;
        this.content = task.content;
        this.deadline = task.deadline;
        this.budget = task.budget;
        this.skills = task.skills;
    }

}