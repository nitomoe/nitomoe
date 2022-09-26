import Toaster from './toaster.js'
import { PostFormData, PostService } from './service/post.js';
import { ThreadFormData, ThreadService } from './service/thread.js';

export default class Sidebar {
    constructor() {
        this._submitButton = document.querySelector('.sidebar .button.submit');
        this._fileBrowseButton = document.querySelector('.sidebar .file-browse');
        this._fileContainer = document.querySelector('.sidebar .file-container');
        this._fileInput = document.querySelector('.sidebar input[name="file"]');
        this._keyInput = document.querySelector('.sidebar input[name="key"]');
        this._subjectInput = document.querySelector('.sidebar input[name="subject"]');
        this._bodyTextarea = document.querySelector('.sidebar textarea[name="body"]');

        this._submitButton.addEventListener('click', this.onSubmitButtonClicked.bind(this));
        this._fileBrowseButton.addEventListener('click', this.onFileBrowseButtonClicked.bind(this));
        this._fileContainer.addEventListener('click', this.onFileBrowseButtonClicked.bind(this));

        this._fileInput.addEventListener('change', this.onFileInputChanged.bind(this));
    }
        
    async onSubmitButtonClicked() {
        const mode = document.querySelector('input[name="mode"]');
        
        if (this._fileInput.files.length > 0) {
            let post = new PostFormData();

            post.board_name = "neet";
            post.key = this._keyInput;
            post.body = this._bodyTextarea;
            post.file = this._fileInput;

            await this.createThread(post);
        }
    }

    /**
     * 
     * @param {Event} event 
     */
    async onFileInputChanged(event) {
        /** @type {HTMLInputElement} */
        const target = event.currentTarget;
        this._fileContainer.innerText = target.files.item(0).name;
    }

    /**
     * @param {PostFormData} post
     */
    async createThread(post) {
        const thread = new ThreadFormData();
        thread.subject = this._subjectInput;
        thread.post = post;

        const onUploadProgress = (event) => {
            const percent = Math.round((event.loaded / event.total) * 100);
            this._submitButton.innerHTML = `${percent}%`;
        };

        const options = {
            onUploadProgress: event => onUploadProgress(event)
        };

        try {
            ThreadService.create(thread, options);
        }
        catch (e) {
            console.error(e);
        }
    }

    onFileBrowseButtonClicked() {
        this._fileInput.click();
    }
}