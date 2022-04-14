#!/usr/bin/env node
import 'source-map-support/register'
import * as cdk from 'aws-cdk-lib'
import { SauronStack } from './sauron_stack'

const app = new cdk.App()
new SauronStack(app, 'SauronStack', {})
