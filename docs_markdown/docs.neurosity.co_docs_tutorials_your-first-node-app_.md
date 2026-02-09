# Source: https://docs.neurosity.co/docs/tutorials/your-first-node-app/

[Skip to main content](https://docs.neurosity.co/docs/tutorials/your-first-node-app/#__docusaurus_skipToContent_fallback)

On this page

Welcome to the Neurosity SDK documentation site. To begin, you'll need to set up an account one time with Neurosity via [console.neurosity.co](https://console.neurosity.co/). Learn how to [create an account with Neurosity Developer Console](https://support.neurosity.co/hc/en-us/articles/360036196792).

## Prerequisites [​](https://docs.neurosity.co/docs/tutorials/your-first-node-app/\#prerequisites "Direct link to Prerequisites")

To download the necessary tools, clone the repository, and install dependencies via `npm`, you need network access.

### NPM [​](https://docs.neurosity.co/docs/tutorials/your-first-node-app/\#npm "Direct link to NPM")

You'll need the following tools:

- [Git](https://git-scm.com/)
- [Node.JS](https://nodejs.org/en/)
- [NPM](https://npmjs.org/), use a [package manager](https://nodejs.org/en/download/package-manager/) to install.

Install and build all of the dependencies using [`NPM`](https://npmjs.org/)

### VSCode [​](https://docs.neurosity.co/docs/tutorials/your-first-node-app/\#vscode "Direct link to VSCode")

We'll be using [VSCode](https://code.visualstudio.com/download) to program this tutorial. For a little added fun, we recommend adding the Neurosity VSCode extension to track your flow state while programming. Check out our guide to [installing and getting started with VSCode and the Neurosity extension](https://support.neurosity.co/hc/en-us/articles/360036195712-Installing-and-using-the-VSCode-extension).

### Tutorial Repository [​](https://docs.neurosity.co/docs/tutorials/your-first-node-app/\#tutorial-repository "Direct link to Tutorial Repository")

Want to see the complete project before reading anymore? You can view all the code from this project in it's [repository on Github](https://github.com/neurosity/app-hello-world-node-js-sdk).

## Setup your Project [​](https://docs.neurosity.co/docs/tutorials/your-first-node-app/\#setup-your-project "Direct link to Setup your Project")

### Hello World Folder [​](https://docs.neurosity.co/docs/tutorials/your-first-node-app/\#hello-world-folder "Direct link to Hello World Folder")

Create a new folder called `hello-world`

```bash
mkdir hello-world
```

Enter into the directory and initialize the `npm` project.

```bash
cd hello-world
npm init
```

You'll need to run through the initial questions:

```bash
package name: (hello-world)
version: (1.0.0)
description: My first application using the Neurosity SDK
entry point: (index.js)
test command:
git repository:
keywords: neurosity
author: Hans Berger
license: (ISC) MIT
```

![Initial set up of NPM project](https://docs.neurosity.co/img/tutorial/npm_init.png)

Next, you'll want to launch a VSCode window for the newly created project.

```bash
code .
```

### Working in VSCode [​](https://docs.neurosity.co/docs/tutorials/your-first-node-app/\#working-in-vscode "Direct link to Working in VSCode")

You'll need to launch a terminal window inside VS Code, you may toggle the terminal with `CTRL+~`.

![Toggle command line](https://docs.neurosity.co/img/tutorial/vscode-toggle-command-line.png)

To create a new file, you may select the new file button.

![Highlighting new file button in vscode](https://docs.neurosity.co/img/tutorial/vscode-new-file-button.png)

Go ahead and make a new file called `index.js`, we'll use it soon as the base of our new project.

![Created a new file called index.js](https://docs.neurosity.co/img/tutorial/vscode-make-index-js-file.png)

## Adding the Neurosity SDK to a Node Project [​](https://docs.neurosity.co/docs/tutorials/your-first-node-app/\#adding-the-neurosity-sdk-to-a-node-project "Direct link to Adding the Neurosity SDK to a Node Project")

### Add `.gitignore` file [​](https://docs.neurosity.co/docs/tutorials/your-first-node-app/\#add-gitignore-file "Direct link to add-gitignore-file")

The first thing we want to do is add a file called `.gitignore` to tell git to ignore certain files. Add another file to the root directory called `.gitignore`, then add the following:

```text
node_modules
```

On MacOS, we'll go ahead and add another commonly ignored file:

```text
.DS_Store
```

![Add a .gitignore file with node_modules](https://docs.neurosity.co/img/tutorial/vscode-gitignore.png)

Adding `node_modules` will help VS Code run a little bit better because we're telling it that we don't need to track anything in that folder.

### Install Dependencies [​](https://docs.neurosity.co/docs/tutorials/your-first-node-app/\#install-dependencies "Direct link to Install Dependencies")

The first dependency we need to install the Neurosity SDK. We'll end up using some environment variables from a `.env` file, so go ahead and install another dependency for that as well. From the command line, enter:

```bash
npm install @neurosity/sdk dotenv
```

![Install dependencies using npm install in the terminal](https://docs.neurosity.co/img/tutorial/vscode-install-dependencies.png)

### Add Dependencies to `index.js` [​](https://docs.neurosity.co/docs/tutorials/your-first-node-app/\#add-dependencies-to-indexjs "Direct link to add-dependencies-to-indexjs")

Importing libraries in Node is quite simple, all you have to do is add the following to the top of your index.js file:

```js
const { Neurosity } = require("@neurosity/sdk");
require("dotenv").config();
```

![Add dependencies to the index.js file](https://docs.neurosity.co/img/tutorial/vscode-add-dependencies-to-index.png)

### Add start script to package.json [​](https://docs.neurosity.co/docs/tutorials/your-first-node-app/\#add-start-script-to-packagejson "Direct link to Add start script to package.json")

Now head over to the file called `package.json`. The `package.json` is at the core of every Node package. **Ignore the file called `package-lock.json`, it's automatically generated.**

Find the section called `"scripts"` and add a property called `"start"` that will start the node process:

```json
"start": "node index.js"
```

Your `package.json` will look like below once added:

```json
{
  "name": "hello-world",
  "version": "1.0.0",
  "description": "My first application using the Neurosity SDK",
  "main": "index.js",
  "scripts": {
    "start": "node index.js",
    "test": "echo \"Error: no test specified\" && exit 1"
  },
  "keywords": ["neurosity"],
  "author": "Hans Berger",
  "license": "MIT",
  "dependencies": {
    "@neurosity/sdk": "^3.8.0",
    "dotenv": "^8.2.0"
  }
}
```

### Run the project from the CLI [​](https://docs.neurosity.co/docs/tutorials/your-first-node-app/\#run-the-project-from-the-cli "Direct link to Run the project from the CLI")

Navigate back to the terminal and run `npm start` to make sure the project runs without any errors.

```bash
npm start
```

You should see the program run and exit successfully.

![Ran our node program with no errors](https://docs.neurosity.co/img/tutorial/vscode-run-empty-program.png)

## Add Authentication [​](https://docs.neurosity.co/docs/tutorials/your-first-node-app/\#add-authentication "Direct link to Add Authentication")

At this point you will need to have [created an account](https://support.neurosity.co/hc/en-us/articles/360036196792-Create-account-with-Neurosity) with [console.neurosity.co](https://console.neurosity.co/) and [claimed your device](https://support.neurosity.co/hc/en-us/articles/360037562351).

### Get variables from `.env` file [​](https://docs.neurosity.co/docs/tutorials/your-first-node-app/\#get-variables-from-env-file "Direct link to get-variables-from-env-file")

We'll first attempt to get our environment variables to show what happens when they are not there at runtime. Add the following code to pull the deviceId, email and password from the enviroment variables:

```js
const deviceId = process.env.DEVICE_ID || "";
const email = process.env.EMAIL || "";
const password = process.env.PASSWORD || "";
```

To verify that the variables are not blank, we could add a function to check for that and quit the program if so. Add the following function to your program next:

```js
const verifyEnvs = (email, password, deviceId) => {
  const invalidEnv = (env) => {
    return env === "" || env === 0;
  };
  if (invalidEnv(email) || invalidEnv(password) || invalidEnv(deviceId)) {
    console.error(
      "Please verify deviceId, email and password are in .env file, quitting..."
    );
    process.exit(0);
  }
};
verifyEnvs(email, password, deviceId);

console.log(`${email} attempting to authenticate to ${deviceId}`);
```

Now, if we run our program, we should see an error print out! Run with `npm start` from the CLI.

![Ran our node program with no envs found error](https://docs.neurosity.co/img/tutorial/vscode-no-env-found.png)

### Add `.env` file [​](https://docs.neurosity.co/docs/tutorials/your-first-node-app/\#add-env-file "Direct link to add-env-file")

Next, we'll add a `.env` to store our deviceId, login, and password. Add a new file called `.env` and add your deviceId, email, and password. Learn how to [find your device ID](https://support.neurosity.co/hc/en-us/articles/360037198152-Get-Notion-Device-ID).

```.env
DEVICE_ID="442333d1bcea35533daba9b51234abcd"
EMAIL="hans.berger@neurosity.co"
PASSWORD="Password#1!"
```

![Created a new file called .env](https://docs.neurosity.co/img/tutorial/vscode-env-file.png)

Now, if we run our program, we should see a success message print out, informing us that our variables have been extracted successfully.

![Pulled out three variables from .env](https://docs.neurosity.co/img/tutorial/vscode-got-env-variables.png)

### Instantiate the Neurosity class [​](https://docs.neurosity.co/docs/tutorials/your-first-node-app/\#instantiate-the-neurosity-class "Direct link to Instantiate the Neurosity class")

We can then use the `deviceId` to instantiate a new Neurosity by adding the following line to our file.

```js
const neurosity = new Neurosity({
  deviceId
});
```

### Add async login [​](https://docs.neurosity.co/docs/tutorials/your-first-node-app/\#add-async-login "Direct link to Add async login")

We need to use an [`async/await`](https://javascript.info/async-await) paradigm for authenticating to the device. Go ahead and create an async function called `main` to the `index.js` file.

```js
const main = async () => {
  await neurosity
    .login({
      email,
      password
    })
    .catch((error) => {
      console.log(error);
      throw new Error(error);
    });
  console.log("Logged in");
};

main();
```

Then run the program with `npm start` in the CLI. If all worked, then you should see:

![Made a function that authenticated with Neurosity](https://docs.neurosity.co/img/tutorial/vscode-main-logged-in.png)

## Add Subscriptions [​](https://docs.neurosity.co/docs/tutorials/your-first-node-app/\#add-subscriptions "Direct link to Add Subscriptions")

### Calm Subscription [​](https://docs.neurosity.co/docs/tutorials/your-first-node-app/\#calm-subscription "Direct link to Calm Subscription")

Now that you are authenticated, print out hello world when you're calm increases past 0.3, a significant number.

Add the following code to your main() function after login.

```js
neurosity.calm().subscribe((calm) => {
  if (calm.probability > 0.3) {
    console.log("Hello World!");
  }
});
```

Your index.js file is now ready to print `Hello World!`

![Add code to subscribe to the calm score](https://docs.neurosity.co/img/tutorial/vscode-main-calm-subscribe.png)

### Kinesis Training [​](https://docs.neurosity.co/docs/tutorials/your-first-node-app/\#kinesis-training "Direct link to Kinesis Training")

Head over to the [Developer Console](https://console.neurosity.co/) and train Left Hand Pinch. [Learn how to train an imagined movement thought](https://support.neurosity.co/hc/en-us/articles/360036344012-Imagined-thought-training). Do at least 15 trials.

When we write code to interact with the Neurosity SDK, we use camel case, so Left Hand Pinch in code is `leftHandPinch`.

Now that the `leftHandPinch` thought is trained, you'll be able to load it into your device for use.

### Kinesis Subscription [​](https://docs.neurosity.co/docs/tutorials/your-first-node-app/\#kinesis-subscription "Direct link to Kinesis Subscription")

In the `index.js` file we can remove the `calm` subscription from above and replace it with the code below.

Check out the [Kinesis guide](https://docs.neurosity.co/docs/api/kinesis) or [Kinesis API docs](https://docs.neurosity.co/docs/reference/interfaces/Kinesis).

```js
neurosity.kinesis("leftHandPinch").subscribe((intent) => {
  console.log("Hello World!");
});
```

Your `index.js` file should look like:

![Add kinesis code to index.js](https://docs.neurosity.co/img/tutorial/vscode-hello-kinesis.png)

## Conclusion [​](https://docs.neurosity.co/docs/tutorials/your-first-node-app/\#conclusion "Direct link to Conclusion")

Developing with the Neurosity SDK can be a lot of fun! There are two main types of thought processes that Neurosity devices can detect: intent and background. The foreground we consider to be the `kinesis()` where you're intending to do something and the background is `calm()` or `focus()` that occurs in the background of the mind.

### Dive into development [​](https://docs.neurosity.co/docs/tutorials/your-first-node-app/\#dive-into-development "Direct link to Dive into development")

We're looking for talented developers to help us improve the kinesis training. So, head over to the [training guide](https://docs.neurosity.co/docs/guides/training) and learn how to build your own training module.

If you're looking for exact API references, check out the [API section](https://docs.neurosity.co/docs/reference) of these docs!

- [Prerequisites](https://docs.neurosity.co/docs/tutorials/your-first-node-app/#prerequisites)
  - [NPM](https://docs.neurosity.co/docs/tutorials/your-first-node-app/#npm)
  - [VSCode](https://docs.neurosity.co/docs/tutorials/your-first-node-app/#vscode)
  - [Tutorial Repository](https://docs.neurosity.co/docs/tutorials/your-first-node-app/#tutorial-repository)
- [Setup your Project](https://docs.neurosity.co/docs/tutorials/your-first-node-app/#setup-your-project)
  - [Hello World Folder](https://docs.neurosity.co/docs/tutorials/your-first-node-app/#hello-world-folder)
  - [Working in VSCode](https://docs.neurosity.co/docs/tutorials/your-first-node-app/#working-in-vscode)
- [Adding the Neurosity SDK to a Node Project](https://docs.neurosity.co/docs/tutorials/your-first-node-app/#adding-the-neurosity-sdk-to-a-node-project)
  - [Add `.gitignore` file](https://docs.neurosity.co/docs/tutorials/your-first-node-app/#add-gitignore-file)
  - [Install Dependencies](https://docs.neurosity.co/docs/tutorials/your-first-node-app/#install-dependencies)
  - [Add Dependencies to `index.js`](https://docs.neurosity.co/docs/tutorials/your-first-node-app/#add-dependencies-to-indexjs)
  - [Add start script to package.json](https://docs.neurosity.co/docs/tutorials/your-first-node-app/#add-start-script-to-packagejson)
  - [Run the project from the CLI](https://docs.neurosity.co/docs/tutorials/your-first-node-app/#run-the-project-from-the-cli)
- [Add Authentication](https://docs.neurosity.co/docs/tutorials/your-first-node-app/#add-authentication)
  - [Get variables from `.env` file](https://docs.neurosity.co/docs/tutorials/your-first-node-app/#get-variables-from-env-file)
  - [Add `.env` file](https://docs.neurosity.co/docs/tutorials/your-first-node-app/#add-env-file)
  - [Instantiate the Neurosity class](https://docs.neurosity.co/docs/tutorials/your-first-node-app/#instantiate-the-neurosity-class)
  - [Add async login](https://docs.neurosity.co/docs/tutorials/your-first-node-app/#add-async-login)
- [Add Subscriptions](https://docs.neurosity.co/docs/tutorials/your-first-node-app/#add-subscriptions)
  - [Calm Subscription](https://docs.neurosity.co/docs/tutorials/your-first-node-app/#calm-subscription)
  - [Kinesis Training](https://docs.neurosity.co/docs/tutorials/your-first-node-app/#kinesis-training)
  - [Kinesis Subscription](https://docs.neurosity.co/docs/tutorials/your-first-node-app/#kinesis-subscription)
- [Conclusion](https://docs.neurosity.co/docs/tutorials/your-first-node-app/#conclusion)
  - [Dive into development](https://docs.neurosity.co/docs/tutorials/your-first-node-app/#dive-into-development)