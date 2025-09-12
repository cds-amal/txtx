import * as path from 'path';
import { runTests } from '@vscode/test-electron';

async function main() {
    try {
        const extensionDevelopmentPath = path.resolve(__dirname, '../../');
        const extensionTestsPath = path.resolve(__dirname, './suite/index');

        // Set up test workspace
        const testWorkspace = path.resolve(__dirname, '../../test-workspace');

        await runTests({
            extensionDevelopmentPath,
            extensionTestsPath,
            launchArgs: [
                testWorkspace,
                '--disable-extensions',
                '--disable-gpu',
                '--no-sandbox'
            ],
            // Add environment variables for headless testing
            extensionTestsEnv: {
                ...process.env,
                DISPLAY: ':99.0',
                CI: 'true'
            }
        });
    } catch (err) {
        console.error('Failed to run tests:', err);
        process.exit(1);
    }
}

main();