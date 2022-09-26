import { api } from '../api.js';

export class PostFormData {
    constructor() {
        this.board_name = undefined;
        this.thread_num = undefined;
        this.key = undefined;
        this.file = undefined;
        this.body = undefined;
    }
}

export class PostService {
    /**
     * Creates a new post
     *
     * @param {PostFormData} post
     * @param {Object} options
     */
    static async create(post, options = {}) {
        const fd = new FormData();
        fd.append('key', post.key);
        fd.append('body', post.body);

        if (post.file) {
            fd.append('file', post.file);
        }

        options = Object.assign({
            body: fd
        }, options);

        return await api.upload(`${post.board_name}/thread/${post.thread_num}/new_reply`, options);
    }
}
