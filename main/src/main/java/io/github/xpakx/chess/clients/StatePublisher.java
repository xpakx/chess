package io.github.xpakx.chess.clients;

import io.github.xpakx.chess.clients.event.StateEvent;
import io.github.xpakx.chess.game.Game;
import io.github.xpakx.chess.game.GameStatus;
import io.github.xpakx.chess.game.GameType;
import io.github.xpakx.chess.game.InvitationStatus;
import org.springframework.amqp.core.AmqpTemplate;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Service;

@Service
public class StatePublisher {
    private final AmqpTemplate template;
    private final String stateTopic;

    public StatePublisher(AmqpTemplate template, @Value("${amqp.exchange.state}") String stateTopic) {
        this.template = template;
        this.stateTopic = stateTopic;
    }

    public void sendGame(Game game) {
        StateEvent event = new StateEvent();
        event.setId(game.getId());
        event.setFinished(game.getStatus() != GameStatus.NotFinished);
        if (event.isFinished()) {
            event.setError(true);
            event.setErrorMessage("Game is already finished!");
        }
        if (game.getInvitation() != InvitationStatus.Accepted) {
            event.setError(true);
            event.setErrorMessage("Game is not accepted!");
        }
        event.setUsername1(game.getUser().getUsername());
        if (game.getOpponent() != null) {
            event.setUsername2(game.getOpponent().getUsername());
        } else {
            event.setUsername2("AI");
        }
        event.setUser2AI(game.getType() == GameType.AI);
        event.setAiType(game.getAiType());
        event.setFirstUserStarts(game.isUserStarts());
        event.setFirstUserTurn(game.isUserTurn());
        event.setCurrentState(game.getCurrentState());
        event.setNonCaptureMoves(game.getNonCaptureMoves());
        template.convertAndSend(stateTopic, "state", event);
    }

    public void sendError(String msg) {
        StateEvent event = new StateEvent();
        event.setError(true);
        event.setErrorMessage(msg);
        template.convertAndSend(stateTopic, "state", event);
    }
}
