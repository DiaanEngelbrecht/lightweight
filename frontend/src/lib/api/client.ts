import { createGrpcWebTransport } from '@connectrpc/connect-web';
import { createAuthInterceptor } from './interceptor';

// Create the transport for communicating with the backend using gRPC-Web protocol
// tonic-web requires the gRPC-Web protocol, not Connect protocol
// Includes auth interceptor to automatically add JWT tokens to requests
export const transport = createGrpcWebTransport({
	baseUrl: 'http://localhost:50570',
	interceptors: [createAuthInterceptor()]
});
