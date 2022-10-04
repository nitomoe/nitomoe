import { api } from '../api.js';
import { PostFormData } from '../service/post.js';

export class ThreadFormData {
    constructor() {
        this.subject = undefined;
        this.post = new PostFormData();
    }
}

export class ThreadService {
    /**
     * Creates a new thread
     *
     * @param {ThreadFormData} thread
     */
    static async create(thread, options = {}) {
        const fd = new FormData();
        fd.append('boardName', thread.post.boardName);
        fd.append('subject', thread.subject);
        fd.append('key', thread.post.key);
        fd.append('body', thread.post.body);

        if (thread.post.file) {
            fd.append('file', thread.post.file);
        }

        options = Object.assign({
            body: fd
        }, options);

        return await api.upload(`/${thread.post.board_name}/new_thread`, options);
    }
}
