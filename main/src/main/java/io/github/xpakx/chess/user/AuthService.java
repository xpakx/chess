package io.github.xpakx.chess.user;

import io.github.xpakx.chess.security.JwtUtils;
import io.github.xpakx.chess.user.dto.AuthenticationRequest;
import io.github.xpakx.chess.user.dto.AuthenticationResponse;
import io.github.xpakx.chess.user.dto.RefreshTokenRequest;
import io.github.xpakx.chess.user.dto.RegistrationRequest;
import io.github.xpakx.chess.user.error.AuthenticationException;
import io.github.xpakx.chess.user.error.ValidationException;
import io.jsonwebtoken.Claims;
import lombok.RequiredArgsConstructor;
import org.springframework.security.authentication.AuthenticationManager;
import org.springframework.security.authentication.BadCredentialsException;
import org.springframework.security.authentication.DisabledException;
import org.springframework.security.authentication.UsernamePasswordAuthenticationToken;
import org.springframework.security.core.userdetails.UserDetails;
import org.springframework.security.crypto.password.PasswordEncoder;
import org.springframework.stereotype.Service;

import java.util.HashSet;
import java.util.Set;

@Service
@RequiredArgsConstructor
public class AuthService {
    private final UserRepository userRepository;
    private final JwtUtils jwtUtils;
    private final UserService userService;
    private final PasswordEncoder passwordEncoder;
    private final AuthenticationManager authenticationManager;
    private final JwtUtils jwt;

    public AuthenticationResponse register(RegistrationRequest request) {
        testRequest(request);
        User user = createNewUser(request);
        authenticate(request.getUsername(), request.getPassword());
        final String token = jwtUtils.generateToken(userService.userAccountToUserDetails(user));
        final String refreshToken = jwtUtils.generateRefreshToken(request.getUsername());
        return AuthenticationResponse.builder()
                .token(token)
                .refreshToken(refreshToken)
                .username(user.getUsername())
                .moderatorRole(false)
                .build();
    }

    private User createNewUser(RegistrationRequest request) {
        Set<UserRole> roles = new HashSet<>();
        User userToAdd = new User();
        userToAdd.setPassword(passwordEncoder.encode(request.getPassword()));
        userToAdd.setUsername(request.getUsername());
        userToAdd.setRoles(roles);
        return userRepository.save(userToAdd);
    }

    private void authenticate(String username, String password) {
        try {
            authenticationManager.authenticate(new UsernamePasswordAuthenticationToken(username, password));
        } catch (DisabledException e) {
            throw new AuthenticationException("User " +username+" disabled!");
        } catch (BadCredentialsException e) {
            throw new AuthenticationException("Invalid password!");
        }
    }

    private void testRequest(RegistrationRequest request) {
        if (userRepository.findByUsername(request.getUsername()).isPresent()) {
            throw new ValidationException("Username exists!");
        }
    }

    public AuthenticationResponse generateAuthenticationToken(AuthenticationRequest authenticationRequest) {
        final UserDetails userDetails = userService.loadUserByUsername(authenticationRequest.getUsername());
        authenticate(authenticationRequest.getUsername(), authenticationRequest.getPassword());
        final String token = jwtUtils.generateToken(userDetails);
        final String refreshToken = jwtUtils.generateRefreshToken(authenticationRequest.getUsername());
        return AuthenticationResponse.builder()
                .token(token)
                .refreshToken(refreshToken)
                .username(authenticationRequest.getUsername())
                .moderatorRole(
                        userDetails.getAuthorities().stream()
                                .anyMatch((a) -> a.getAuthority().equals("MODERATOR"))
                )
                .build();
    }
}
