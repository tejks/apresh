import type { Meta, StoryObj } from '@storybook/svelte';
import Modal from './Modal.svelte';

// More on how to set up stories at: https://storybook.js.org/docs/writing-stories
const meta = {
	title: 'Modal/Modal',
	component: Modal,
	tags: ['autodocs'],
	argTypes: {}
} satisfies Meta<Modal>;

export default meta;
type Story = StoryObj<typeof meta>;

const defaultArgs = {
	showModal: true,
	showClose: true,
	closeHandler: () => {
		console.log('closeHandler');
	}
};

// TODO: Pass children to Modal.
// More on writing stories with args: https://storybook.js.org/docs/writing-stories/args
export const Basic: Story = { args: { ...defaultArgs } };
// export const WithoutClose: Story = { args: { ...defaultArgs, showClose: false } };
// export const Hidden: Story = { args: { ...defaultArgs, showModal: false } };
